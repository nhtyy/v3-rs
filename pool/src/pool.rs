use crate::numeraire::PoolPrice;
pub use ethers::types::{Address, I256, U256};
use rug::{float::ParseFloatError, ops::Pow, Float};
use std::collections::{hash_map::Entry, HashMap};
use std::ops::DerefMut;
use std::{error::Error, ops::Deref};

pub type Tick = i32;

#[derive(Debug, Clone, Copy)]
pub enum V3PoolError<E: Error> {
    ParseError(ParseFloatError),
    BackendError(E),
    BadRange(Tick, Tick, i32),
}

/// [Deltas] is a simple struct that holds some deltas of token0 and token1
#[derive(Debug, Clone)]
pub struct Deltas {
    token0: Address,
    token1: Address,
    amounts: HashMap<Address, Float>,
}

impl Deref for Deltas {
    type Target = HashMap<Address, Float>;

    fn deref(&self) -> &Self::Target {
        &self.amounts
    }
}

impl DerefMut for Deltas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amounts
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FeeTier {
    Min = 500,
    Mid = 3000,
    Max = 10000,
}

impl FeeTier {
    pub fn as_u24_bytes(&self) -> Vec<u8> {
        let temp = match self {
            FeeTier::Min => 500_u64.to_be_bytes(),
            FeeTier::Mid => 3000_u64.to_be_bytes(),
            FeeTier::Max => 10000_u64.to_be_bytes(),
        };

        temp[5..].to_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TickSpacing {
    Min = 10,
    Mid = 60,
    Max = 200,
}

/// [SwapMath] is implemented on all [V3Pool]s as well as some other types
pub trait SwapMath {
    fn token0_delta(liquidty: Float, sqrt_price: Float, target_price: Float) -> Float {
        let one = Float::with_val(100, 1);
        let invert_target = one.clone() / target_price;
        let invert_price = one / sqrt_price;

        liquidty * (invert_target - invert_price)
    }

    fn token1_delta(liquidty: Float, sqrt_price: Float, target_price: Float) -> Float {
        liquidty * (target_price - sqrt_price)
    }
}

impl SwapMath for Deltas {}

// apply the trait to all types that implement V3Pool
impl<P: V3Pool> SwapMath for P {}

impl Deltas {
    pub fn new(token0: Address, token1: Address) -> Self {
        let mut map: HashMap<Address, Float> = HashMap::new();
        map.insert(token0, Float::with_val(100, 0));
        map.insert(token1, Float::with_val(100, 0));

        Self {
            token0,
            token1,
            amounts: map,
        }
    }

    pub fn update(&mut self, liquidity: Float, sqrt_price: Float, target_price: Float) {
        let token0_entry = self
            .amounts
            .get_mut(&self.token0)
            .expect("token0 should exist");

        *token0_entry +=
            Self::token0_delta(liquidity.clone(), sqrt_price.clone(), target_price.clone());

        let token1_entry = self
            .amounts
            .get_mut(&self.token1)
            .expect("token1 should exist");

        *token1_entry += Self::token1_delta(liquidity, sqrt_price, target_price);
    }

    pub fn apply_fee(&mut self, fee: u64) {
        self.values_mut().for_each(|v| {
            if !v.is_sign_negative() {
                let fee = Float::with_val(100, fee);
                tracing::info!("applying fee: {}", &fee);
                let decay = 1 - fee;
                tracing::info!("decay: {}", &decay);
                *v /=  decay;
            }
        })
    }
}

impl FeeTier {
    pub fn as_spacing(&self) -> TickSpacing {
        match self {
            FeeTier::Min => TickSpacing::Min,
            FeeTier::Mid => TickSpacing::Mid,
            FeeTier::Max => TickSpacing::Max,
        }
    }

    pub fn as_bp(&self) -> u64 {
        match self {
            FeeTier::Min => 5,
            FeeTier::Mid => 30,
            FeeTier::Max => 100,
        }
    }
}

impl TickSpacing {
    pub fn as_fee(tick_spacing: TickSpacing) -> FeeTier {
        match tick_spacing {
            TickSpacing::Min => FeeTier::Min,
            TickSpacing::Mid => FeeTier::Mid,
            TickSpacing::Max => FeeTier::Max,
        }
    }
}

/// [V3Pool] is the main trait that all v3 pools should implement
/// it provides a set of functions that can be used to calculate the price of the pool
/// as well as the amount of token0 and token1 needed to move the pool price to a target price
///
/// it also provides some low level price functionality, that is built upon by other traits such as [crate::numeraire::Numeraire]
#[async_trait::async_trait]
pub trait V3Pool: SwapMath + Sized {
    type Ticks: IntoIterator<Item = Float>;
    type BackendError: Error;

    fn fee(&self) -> FeeTier;
    fn token0(&self) -> Address;
    fn token1(&self) -> Address;
    fn token0_decimals(&self) -> u8;
    fn token1_decimals(&self) -> u8;

    async fn current_liquidity(&self) -> Result<Float, V3PoolError<Self::BackendError>>;
    async fn sqrt_price_x96(&self) -> Result<U256, V3PoolError<Self::BackendError>>;

    /// returns the liqudity delta at a given value in terms of crossing from left to right
    /// in other words if the price of the pool is increasing, this tick delta will should be added
    async fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>>;

    /// since tick delta should be added as price increase, a tick range can account for the opposite case
    /// if ending < starting, you can flip the signs of the deltas, implementors should ensure that you can always add these values
    /// we also let implementors handle this because they may have some optimizations for this case
    async fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> Result<Self::Ticks, V3PoolError<Self::BackendError>>;

    fn tick_spacing(&self) -> TickSpacing {
        self.fee().as_spacing()
    }

    fn x96() -> Float {
        Float::with_val(100, 2u128.pow(96))
    }

    async fn sqrt_price(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let valid = Float::parse(self.sqrt_price_x96().await?.to_string())?;

        let price = Float::with_val(100, valid);

        Ok(price / Self::x96())
    }

    /// represnets the LOWER TICK of a given price
    /// in other words this function rounds down with respect to the tick spacing
    fn price_to_tick(price: Float, tick_spacing: TickSpacing) -> Tick {
        let spacing = Float::with_val(100, tick_spacing as i32);

        // change of base log[1.0001](price)
        let log_10001_price = price.ln() / Float::with_val(100, 1.0001).ln();

        let spacing_scalar = log_10001_price / &spacing;

        (spacing_scalar.floor() * spacing)
            .to_i32_saturating_round(rug::float::Round::Down)
            .expect("Failed to convert tick to i32")
    }

    /// 1.0001^tick
    fn tick_to_price(tick: Tick) -> Float {
        let base = Float::with_val(100, 1.0001);
        base.pow(tick)
    }

    /// Returns the amount of token0 and token1 needed to move the pool price to the target price
    /// price_of_0_in_1 should not include the underlying nominal units
    async fn amounts_to_move_price(
        &self,
        new_price: PoolPrice<'async_trait, Self>,
    ) -> Result<Deltas, V3PoolError<Self::BackendError>> {
        let mut spacing = self.tick_spacing() as i32;
        let mut current_liquidity = self.current_liquidity().await?;

        tracing::debug!("current L: {:?}", current_liquidity);

        let mut current_sqrt_price = self.sqrt_price().await?;
        let target_sqrt_price = new_price.into_pool_price_float().sqrt();

        tracing::debug!("current sqrt price: {:?}", current_sqrt_price);
        tracing::debug!("target sqrt price: {:?}", target_sqrt_price);

        let current_lower_tick = Self::price_to_tick(self.sqrt_price().await?, self.tick_spacing());
        let target_lower_tick = Self::price_to_tick(target_sqrt_price.clone(), self.tick_spacing());

        let mut deltas = Deltas::new(self.token0(), self.token1());
        let mut next_tick: i32 = Default::default();

        let ticks = if current_lower_tick < target_lower_tick {
            tracing::debug!("current lower tick is less than target lower tick");
            // ending will be the lower tick of where the target price is
            // starting will be the upper tick of the current price is

            next_tick = current_lower_tick + spacing;

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                Self::tick_to_price(next_tick),
            );

            self.tick_range(next_tick, target_lower_tick).await?
        } else if current_lower_tick > target_lower_tick {
            // ending will be the upper tick of where the target price is
            // starting will be the lower tick of the current price

            next_tick = current_lower_tick;

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                Self::tick_to_price(next_tick),
            );

            let ticks = self
                .tick_range(next_tick, target_lower_tick + spacing)
                .await?;

            spacing = -spacing;

            ticks
        } else {
            tracing::debug!("current lower tick is equal to target lower tick");
            // equal case
            self.tick_range(current_lower_tick, current_lower_tick)
                .await?
        };

        let mut ticks = ticks.into_iter();
        while let Some(delta) = ticks.next() {
            let current_tick = next_tick;

            current_liquidity += delta;
            next_tick += spacing;
            current_sqrt_price = Self::tick_to_price(current_tick);

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                Self::tick_to_price(next_tick),
            );
        }

        deltas.update(
            current_liquidity.clone(),
            current_sqrt_price,
            target_sqrt_price.clone(),
        );

        deltas.apply_fee(self.fee().as_bp());

        Ok(deltas)
    }
}

impl<T: Error> std::error::Error for V3PoolError<T> {}

impl<T: Error> std::fmt::Display for V3PoolError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            V3PoolError::ParseError(e) => write!(f, "V3PoolError::ParseError {}", e),
            V3PoolError::BackendError(e) => write!(f, "V3PoolError::BackendError {}", e),
            V3PoolError::BadRange(starting, ending, spacing) => write!(
                f,
                "V3PoolError::BadRange starting: {}, ending: {}, spacing: {}",
                starting, ending, spacing
            ),
        }
    }
}

impl<T: Error> From<ParseFloatError> for V3PoolError<T> {
    fn from(e: ParseFloatError) -> Self {
        V3PoolError::ParseError(e)
    }
}
