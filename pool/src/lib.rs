#[macro_use]
mod macros;

mod traits;

/// Addresses from Uniswap and things like that
pub mod constants;

/// an implementation of the [crate::pool::V3Pool] trait
pub mod ethers_pool;

pub mod pool;
pub use pool::{V3Pool, PoolResult, liquidity::LiquidityExt, swap::SwapExt, price::PriceExt};

pub mod position;
pub use position::PositionsReturn as Position;
pub use position::PositionManager;

pub mod math;

pub mod error;

pub mod types;
pub use types::{FeeTier, TickSpacing, Token, price::PoolPrice, amount::TokenAmount};
