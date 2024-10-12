pub mod price;
pub mod swap;

use std::error::Error;

use crate::error::V3PoolError;
use crate::math::{Price, SqrtPrice, Tick};

use crate::PoolPrice;
use crate::TokenIdx;
use alloy::primitives::{Signed, Uint};
use alloy::primitives::Address;
use lazy_static::lazy_static;
use rug::Float;

pub type PoolResult<T, E> = std::result::Result<T, V3PoolError<E>>;

lazy_static! {
    pub static ref X96: Float = Float::with_val(100, 2u128.pow(96));
}

/// [V3Pool] is the main trait of this library. It encompasses all the low level functions of a uniswapv3 pool
#[async_trait::async_trait]
pub trait V3Pool: Send + Sync + Sized {
    type BackendError: Error + Send + Sync;

    /// The fee tier of the pool
    fn fee(&self) -> Uint<24, 1>;
    fn token0(&self) -> &Address;
    fn token0_decimals(&self) -> &u8;
    fn token1(&self) -> &Address;
    fn token1_decimals(&self) -> &u8;
    fn address(&self) -> Address;

    // The current in range liquidity of the pool
    async fn current_liquidity(&self) -> PoolResult<Float, Self::BackendError>;

    /// The sqrt price of the pool scaled by 2^96
    async fn sqrt_price_x96(&self) -> PoolResult<Float, Self::BackendError>;

    /// Returns the liqudity delta to be added if youre were crossing
    /// into this tick as price is increasing
    async fn tick(&self, tick: Tick) -> PoolResult<Float, Self::BackendError>;

    /// returns the deltas (accounting for direction) for ticks `[starting, ending)`
    /// if starting == ending, returns []
    /// ### Notice:
    /// implementors should ensure that the returned amount is correct for the direction
    /// Since tick delta should be added as price increase, a tick range can account for the opposite case
    /// if ending < starting, you can flip the signs of the deltas
    async fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> PoolResult<Vec<i128>, Self::BackendError>;

    /// Returns the position of the token in the pool
    fn position_of(&self, token: &Address) -> Option<TokenIdx> {
        if token == self.token0() {
            Some(TokenIdx::Zero)
        } else if token == self.token1() {
            Some(TokenIdx::One)
        } else {
            None
        }
    }

    /// Returns the tick spacing of the pool
    fn tick_spacing(&self) -> Signed<24, 1>;

    /// The price of the pool (with decimals)
    async fn price(&self) -> PoolResult<Price, Self::BackendError> {
        Ok(self.sqrt_price().await?.into())
    }

    /// The sqrt price of the pool (with decimals)
    async fn sqrt_price(&self) -> PoolResult<SqrtPrice, Self::BackendError> {
        // saftey: sqrt price comes from pool
        Ok(unsafe { SqrtPrice::new_unchecked(self.sqrt_price_x96().await? / &*X96) })
    }

    /// Returns the current [crate::PoolPrice] in terms of the numeraire
    async fn pool_price(
        &self,
        numeraire: TokenIdx,
    ) -> PoolResult<PoolPrice<'_, Self>, Self::BackendError> {
        Ok(PoolPrice::from_price(self, self.price().await?, numeraire))
    }
}
