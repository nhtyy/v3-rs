use super::pool::Pool;
use super::pool::V3PoolContract::V3PoolContractInstance;
use crate::{error::V3PoolError, FeeTier};
use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use FactoryInterface::FactoryInterfaceInstance;

alloy::sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    interface FactoryInterface {
        /// @notice Returns the pool address for a given pair of tokens and a fee, or address 0 if it does not exist
        /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
        /// @param tokenA The contract address of either token0 or token1
        /// @param tokenB The contract address of the other token
        /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
        /// @return pool The pool address
        function getPool(
            address tokenA,
            address tokenB,
            uint24 fee
        ) external view returns (address pool);
    }

    #[sol(rpc)]
    #[cfg(feature = "aerodrome")]
    interface AerodromeInterface {
        /// @notice Returns the pool address for a given pair of tokens and a fee, or address 0 if it does not exist
        /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
        /// @param tokenA The contract address of either token0 or token1
        /// @param tokenB The contract address of the other token
        /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
        /// @return pool The pool address
        function getPool(
            address tokenA,
            address tokenB,
            int24 tickSpacing
        ) external view returns (address pool);
    }
}

/// The alloy implementation of an on chain v3 factory.
/// 
/// This is useful for getting pool instances from the underlying tokens
pub struct Factory<T, P, N> {
    instance: FactoryInterfaceInstance<T, P, N>,
}

impl<T, P, N> Factory<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub const fn new(address: Address, provider: P) -> Self {
        Self { instance: FactoryInterfaceInstance::new(address, provider) }
    }

    pub async fn pool_address(
        &self,
        first_token: Address,
        second_token: Address,
        fee: FeeTier,
    ) -> Result<Address, alloy::contract::Error> {
        self.instance.getPool(first_token, second_token, fee.as_scaled_bp())
            .call()
            .await
            .map(|x| x.pool)
    }

    /// todo! maybe we spawn a thread here and send the Pool over a channel or use an arc to share
    /// the pool between threads.
    ///
    /// should be ran inside its own thread as it blocks whatever thread it was instantiated on
    pub async fn pool(
        &self,
        first_token: Address,
        second_token: Address,
        fee: FeeTier,
    ) -> Result<Pool<T, &P, N>, V3PoolError<alloy::contract::Error>> {
        self.pool_with_provider(first_token, second_token, fee, self.instance.provider())
            .await
    }

    pub async fn pool_with_provider<P2>(
        &self,
        first_token: Address,
        second_token: Address,
        fee: FeeTier,
        provider: P2,
    ) -> Result<Pool<T, P2, N>, V3PoolError<alloy::contract::Error>>
    where
        P2: Provider<T, N>,
    {
        let address = self
            .pool_address(first_token, second_token, fee)
            .await
            .map_err(V3PoolError::backend_error)?;

        if address == Address::ZERO {
            return Err(V3PoolError::PoolNotFound);
        }

        let bindings = V3PoolContractInstance::new(address, provider);

        Pool::new(bindings)
            .await
            .map_err(V3PoolError::backend_error)
    }
}

#[cfg(feature = "aerodrome")]
pub use aerodrome::{
    AerodromeFactory,
    AERODROME_FACTORY_ADDRESS
};

#[cfg(feature = "aerodrome")]
mod aerodrome {
    
    use super::*;
    use super::AerodromeInterface::AerodromeInterfaceInstance as IAerodromeFactory;
    use alloy::contract::Error as ContractError;
    use alloy::primitives::Signed;

    pub const AERODROME_FACTORY_ADDRESS: Address = alloy::primitives::address!("5e7BB104d84c7CB9B682AaC2F3d509f5F406809A");

    pub struct AerodromeFactory<T, P, N> {
        inner: IAerodromeFactory<T, P, N>,
    }
    
    impl<T, P, N> AerodromeFactory<T, P, N>
    where
        P: Provider<T, N>,
        T: Transport + Clone,
        N: Network,
    {
        pub fn new(provider: P) -> Self {
            Self {
                inner: IAerodromeFactory::new(AERODROME_FACTORY_ADDRESS, provider),
            }
        }
    
        pub async fn get_pool_address(
            &self,
            token_a: Address,
            token_b: Address,
            tick_spacing: Signed<24, 1>,
        ) -> Result<Address, ContractError> {    
            let addr = self.inner
                .getPool(token_a, token_b, tick_spacing)
                .call()
                .await
                .map(|addr| addr.pool)?;
    
            Ok(addr)
        }
    
        pub async fn get_pool(
            &self,
            token_a: Address,
            token_b: Address,
            tick_spacing: Signed<24, 1>,
        ) -> anyhow::Result<crate::AerodromePool<T, &P, N>> {
            self.get_pool_with_provider(self.inner.provider(), token_a, token_b, tick_spacing)
                .await
        }
    
        pub async fn get_pool_with_provider<P2>(
            &self,
            provider: P2,
            token_a: Address,
            token_b: Address,
            tick_spacing: Signed<24, 1>,
        ) -> anyhow::Result<crate::AerodromePool<T, P2, N>>
        where
            P2: Provider<T, N>,
        {
            let addr = self
                .get_pool_address(token_a, token_b, tick_spacing)
                .await?;
    
            let instance =
                crate::alloy_pool::pool::V3PoolContract::V3PoolContractInstance::new(addr, provider);
    
            Ok(crate::AerodromePool::new(instance).await?)
        }
    }    
}

