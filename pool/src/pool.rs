pub mod liquidity;
pub mod price;
pub mod swap;

use crate::error::V3PoolError;
use crate::math::{SqrtPrice, Tick};

pub use ethers::types::{Address, I256, U256};
use futures::Stream;
use rug::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Zero,
    One,
}

/// A result type for pool operations
pub type PoolResult<T, E> = std::result::Result<T, V3PoolError<E>>;

#[derive(Debug, Clone, Copy)]
pub enum FeeTier {
    Min,
    Mid,
    Max,
}

impl FeeTier {
    /// Big endian representation of the fee tier
    pub const fn as_u24_bytes(&self) -> [u8; 3] {
        match self {
            FeeTier::Min => [0, 1, 244],
            FeeTier::Mid => [0, 11, 184],
            FeeTier::Max => [0, 39, 16],
        }
    }

    pub const fn as_spacing(&self) -> TickSpacing {
        match self {
            FeeTier::Min => TickSpacing::Min,
            FeeTier::Mid => TickSpacing::Mid,
            FeeTier::Max => TickSpacing::Max,
        }
    }

    pub const fn as_bp(&self) -> u32 {
        match self {
            FeeTier::Min => 5,
            FeeTier::Mid => 30,
            FeeTier::Max => 100,
        }
    }

    pub const fn as_scaled_bp(&self) -> u32 {
        match self {
            FeeTier::Min => 500,
            FeeTier::Mid => 3000,
            FeeTier::Max => 10000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TickSpacing {
    Min = 10,
    Mid = 60,
    Max = 200,
}

impl TickSpacing {
    pub const fn as_fee(tick_spacing: TickSpacing) -> FeeTier {
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
pub trait V3Pool: Send + Sync + Sized + 'static {
    type BackendError: std::error::Error + Send + Sync + 'static;

    /// An ordred stream of ticks
    type Ticks: Stream<Item = Result<Float, Self::BackendError>> + Send + Sync + 'static;

    fn fee(&self) -> &FeeTier;
    fn token0(&self) -> &Address;
    fn token1(&self) -> &Address;
    fn token0_decimals(&self) -> &u8;
    fn token1_decimals(&self) -> &u8;

    fn x96() -> Float {
        Float::with_val(100, 2u128.pow(96))
    }

    fn tick_spacing(&self) -> TickSpacing {
        self.fee().as_spacing()
    }

    /// The sqrt price of the pool
    async fn sqrt_price(&self) -> Result<SqrtPrice, V3PoolError<Self::BackendError>> {
        // saftey: sqrt price comes from pool
        Ok(unsafe { SqrtPrice::new_unchecked(self.sqrt_price_x96().await? / Self::x96()) })
    }

    // The current in range liquidity of the pool
    async fn current_liquidity(&self) -> PoolResult<Float, Self::BackendError>;

    /// The sqrt price of the pool scaled by 2^96
    async fn sqrt_price_x96(&self) -> PoolResult<Float, Self::BackendError>;

    /// Returns the liqudity delta to be added if youre were crossing
    /// into this tick as price is increasing
    async fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>>;

    /// Since tick delta should be added as price increase, a tick range can account for the opposite case
    /// if ending < starting, you can flip the signs of the deltas
    ///
    /// SO: implementors should ensure that the returned amount is correct for the direction
    ///
    /// returns the deltas (accounting for direction) between [starting, ending]
    /// if starting == ending, returns []
    fn tick_range(&self, starting: Tick, ending: Tick) -> Self::Ticks;
}
