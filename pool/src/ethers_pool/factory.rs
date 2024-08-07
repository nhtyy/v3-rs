use super::pool::Pool;
use crate::{error::V3PoolError, FeeTier};
use bindings::{V3FactoryContract, V3PoolContract};
use ethers::{contract::ContractError, providers::Middleware, types::Address};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Factory<M: Middleware + 'static> {
    middleware: Arc<M>,
    factory: V3FactoryContract<M>,
}

impl<M: Middleware + 'static> Factory<M> {
    pub fn new(factory_address: Address, middleware: std::sync::Arc<M>) -> Self {
        Self {
            factory: V3FactoryContract::new(factory_address, middleware.clone()),
            middleware,
        }
    }

    pub async fn pool_address(
        &self,
        first_token: Address,
        second_token: Address,
        fee: FeeTier,
    ) -> Result<Address, V3PoolError<ContractError<M>>> {
        self.factory
            .get_pool(first_token, second_token, fee.as_scaled_bp())
            .call()
            .await
            .map_err(V3PoolError::backend_error)
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
    ) -> Result<Pool<M>, V3PoolError<ContractError<M>>> {
        let address = self.pool_address(first_token, second_token, fee).await?;

        if address == Address::zero() {
            return Err(V3PoolError::PoolNotFound);
        }

        let bindings = V3PoolContract::new(address, self.middleware.clone());

        Pool::new(bindings, self.middleware.clone(), fee)
            .await
            .map_err(V3PoolError::backend_error)
    }
}
