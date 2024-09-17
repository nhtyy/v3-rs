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
}

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

        Pool::new(bindings, fee)
            .await
            .map_err(V3PoolError::backend_error)
    }
}
