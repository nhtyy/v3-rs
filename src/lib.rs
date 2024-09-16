#[macro_use]
mod macros;

pub mod traits;

/// Addresses from Uniswap and things like that
pub mod constants;

pub mod alloy_pool;
pub use alloy_pool::{factory::Factory as AlloyFactory, pool::Pool as AlloyPool};

pub mod pool;
pub use pool::{price::PriceExt, PoolResult, V3Pool};

mod position;
pub use position::{Manager, PositionsReturn};

pub mod error;
pub mod math;

pub mod types;
pub use types::{amount::TokenAmount, price::PoolPrice, FeeTier, TickSpacing, Token};

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