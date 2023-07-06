use crate::pool::{Deltas, V3Pool, V3PoolError};
use rug::{ops::Pow, Float};

pub enum Token {
    Zero,
    One,
}

pub struct PoolPrice<'pool, P: V3Pool> {
    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Float,

    /// the pool that this price is for
    pool: &'pool P,
}

impl<'pool, P: V3Pool> PoolPrice<'pool, P> {
    /// creates a new [PoolPrice] from a pool and a price
    /// the price should not attempt to account for decimals
    pub fn new(pool: &'pool P, price: Float, numeraire: Token) -> Self {
        match numeraire {
            Token::One => Self { price, pool },
            Token::Zero => Self {
                price: Float::with_val(100, 1) / price,
                pool,
            },
        }
    }

    /// consumes the type and returns the pool price accounting for the decimals of the pool
    pub fn into_pool_price_float(self) -> Float {
        let exp = self.pool.token1_decimals() as i16 - self.pool.token0_decimals() as i16;

        self.price * Float::with_val(100, 10).pow(exp)
    }

    /// returns the human readable price in terms of some numeriare
    pub fn price_in(&self, numeraire: Token) -> Float {
        match numeraire {
            Token::One => self.price.clone(),
            Token::Zero => Float::with_val(100, 1) / self.price.clone(),
        }
    }
}

/// [Numeraire] is a trait that is implemented on all [V3Pool]s
///
/// It provides some helper functions for getting the price of the pool in "human readable" form
///
/// as well as provides an interface for fetching this in some target quote currency
///
/// todo we should probaly have strong types on the Prices returned
pub trait Numeraire: V3Pool + Sized {
    fn pool_price(&self) -> Result<PoolPrice<Self>, V3PoolError<Self::BackendError>> {
        let price = self.sqrt_price()?.pow(2);

        let exp = -(self.token1_decimals() as i16 - self.token0_decimals() as i16);

        let price = price * Float::with_val(100, 10).pow(exp);

        Ok(PoolPrice::new(self, price, Token::One))
    }

    /// Returns the amount of token0 and token1 needed to move the pool price to the target price
    /// price_of_0_in_1 should not include the underlying nominal units
    fn amounts_to_move_price(
        &self,
        new_price: PoolPrice<Self>,
    ) -> Result<Deltas, V3PoolError<Self::BackendError>> {
        let mut spacing = self.tick_spacing() as i32;
        let mut current_liquidity = self.current_liquidity()?;
        let current_sqrt_price = self.sqrt_price()?;
        let target_sqrt_price = new_price.into_pool_price_float().sqrt();

        let current_lower_tick = Self::price_to_tick(self.sqrt_price()?, self.tick_spacing());
        let target_lower_tick = Self::price_to_tick(target_sqrt_price.clone(), self.tick_spacing());

        let mut deltas = Deltas::new();
        let mut next_tick: i32 = Default::default();

        let ticks = if current_lower_tick < target_lower_tick {
            // ending will be the lower tick of where the target price is
            // starting will be the upper tick of the current price is

            next_tick = current_lower_tick + spacing;

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price,
                Self::tick_to_price(next_tick),
            );

            self.tick_range(next_tick, target_lower_tick)?
        } else if current_lower_tick > target_lower_tick {
            // ending will be the upper tick of where the target price is
            // starting will be the lower tick of the current price

            next_tick = current_lower_tick;

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price,
                Self::tick_to_price(next_tick),
            );

            let ticks = self.tick_range(current_lower_tick, target_lower_tick + spacing)?;

            spacing = -spacing;

            ticks
        } else {
            // equal case
            self.tick_range(current_lower_tick, current_lower_tick)?
        };

        let mut ticks = ticks.into_iter().peekable();
        loop {
            match ticks.peek() {
                Some(_) => {
                    let delta = ticks.next().expect("peeked value should exist");
                    let current_tick = next_tick;

                    current_liquidity += delta;
                    next_tick += spacing;

                    deltas.update(
                        current_liquidity.clone(),
                        Self::tick_to_price(current_tick),
                        Self::tick_to_price(next_tick),
                    );
                }
                None => {
                    deltas.update(
                        current_liquidity.clone(),
                        Self::tick_to_price(next_tick),
                        target_sqrt_price,
                    );

                    return Ok(deltas);
                }
            }
        }
    }
}

impl<P: V3Pool> Numeraire for P {}
