pub mod liquidity;
pub mod swap;
pub mod tick;
mod wrappers;

use rug::Float;

use ethers::types::U256;
pub use wrappers::{Price, SqrtPrice, Tick, BoundsError};

pub trait IntoFloat {
    fn into_float(self) -> Float;
}

impl IntoFloat for U256 {
    fn into_float(self) -> Float {
        // safey: U256 is always a valid float
        let parse = Float::parse(self.to_string()).expect("Failed to parse U256 to Float");

        Float::with_val(100, parse)
    }
}
