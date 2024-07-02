pub mod liquidity;
pub mod swap;
pub mod tick;

/// A set of type wrappers for common items found in UniswapV3
/// 
/// Theses wrappers are guarnteed to be within the bounds allowed by the protocol
/// and are checked at runtime unless unsafe code is used
mod wrappers;

use rug::Float;

pub use wrappers::{Price, SqrtPrice, Tick, BoundsError};