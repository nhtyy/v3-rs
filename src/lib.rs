#[macro_use]
mod macros;

pub mod traits;

/// Addresses from Uniswap and things like that
pub mod constants;

pub mod alloy_pool;

pub mod pool;
pub use pool::{V3Pool, PoolResult, price::PriceExt};

mod position;
pub use position::PositionsReturn;
/// The UniV3 NFT position manager
pub use position::Manager;

pub mod math;

pub mod error;

pub mod types;
pub use types::{FeeTier, TickSpacing, Token, price::PoolPrice, amount::TokenAmount};

mod utils {
    use alloy::providers::Provider;
    use alloy::transports::Transport;
    use alloy::network::Network;
    use alloy::primitives::Address;
    use alloy::contract::Error;
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
        N: Network
    {
        let instance = ERC20Instance::new(address, provider);

        instance.decimals().call().await.map(|v| v._0)
    }
}