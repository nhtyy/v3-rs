#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

pub mod traits;

/// Addresses from Uniswap and things like that
pub mod constants;

/// The Alloy Implementation of the [crate::V3Pool] trait
pub mod alloy_pool;
#[doc(inline)]
pub use alloy_pool::{
    factory::Factory,
    pool::Pool,
};

#[cfg(feature = "aerodrome")]
pub use alloy_pool::{pool::AerodromePool, factory::AerodromeFactory};

pub mod pool;
#[doc(inline)]
pub use pool::{price::PriceExt, PoolResult, V3Pool};

mod position;
#[doc(inline)]
pub use position::{Manager as AlloyManager, PositionsReturn};

/// The error type that is returned from interacting with a [crate::V3Pool]
pub mod error;

/// The math module contains all the math functions that are used in the library
pub mod math;

/// Some useful types that are used in the library
pub mod types;
#[doc(inline)]
pub use types::{FeeTier, PoolPrice, TickSpacing, Token as TokenIdx, TokenAmount};

pub type I24 = alloy::primitives::Signed<24, 1>;

mod utils {
    use alloy::contract::Error;
    use alloy::network::Network;
    use alloy::primitives::Address;
    use alloy::providers::Provider;
    use alloy::transports::Transport;
    use ERC20::ERC20Instance;

    alloy::sol! {
        #[derive(Debug)]
        #[sol(rpc)]
        interface ERC20 {
            function decimals() external view returns (uint8);
        }
    }

    pub async fn decimals<T, N, P>(provider: P, address: Address) -> Result<u8, Error>
    where
        T: Transport + Clone,
        P: Provider<T, N>,
        N: Network,
    {
        let instance = ERC20Instance::new(address, provider);

        instance.decimals().call().await.map(|v| v._0)
    }
}
