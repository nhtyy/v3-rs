pub mod liquidity;
pub mod price;
pub mod swap;

use crate::error::V3PoolError;
use crate::math::{SqrtPrice, Tick};

use crate::{FeeTier, TickSpacing};
use ethers::types::Address;
use futures::Stream;
use rug::Float;

pub type PoolResult<T, E> = std::result::Result<T, V3PoolError<E>>;

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
