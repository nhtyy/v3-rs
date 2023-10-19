use super::pool::Pool;
use crate::pool::{FeeTier, V3PoolError};
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
        Ok(self
            .factory
            .get_pool(first_token, second_token, fee as u32)
            .call()
            .await?)
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

        let bindings = V3PoolContract::new(address, self.middleware.clone());

        Ok(Pool::new(bindings, self.middleware.clone(), fee).await?)
    }
}
