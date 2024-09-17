//! V3-rs is a Rust library for interacting with Uniswap V3 Pools.
//! 
//! V3-rs currently has functionality for:
//! computing the optimal swap amount for a given price/pool 
//! 
//! and 
//! 
//! computing the LP balances owned by a given address (NFT positions only)
//! 
//! We also expose some useful [crate::types] that can help you seemlsly interact with on chain pools without having to think about token decimals.

#[macro_use]
mod macros;

pub mod traits;

/// Addresses from Uniswap and things like that
pub mod constants;

/// The Alloy Implementation of the [crate::V3Pool] trait
pub mod alloy_pool;
pub use alloy_pool::{factory::Factory as AlloyFactory, pool::Pool as AlloyPool};

pub mod pool;
pub use pool::{price::PriceExt, PoolResult, V3Pool};

mod position;
pub use position::{Manager as AlloyManager, PositionsReturn};

/// The error type that is returned from interacting with a [crate::V3Pool]
pub mod error;

/// The math module contains all the math functions that are used in the library
pub mod math;

/// Some useful types that are used in the library
pub mod types;
pub use types::{TokenAmount, PoolPrice, FeeTier, TickSpacing, Token as TokenIdx};

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
