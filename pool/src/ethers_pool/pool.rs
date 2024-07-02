use std::pin::Pin;

use crate::error::V3PoolError;
use crate::math::Tick;
use crate::traits::IntoFloat;
// use crate::pool::{FeeTier, Tick, TickSpacing, V3Pool, V3PoolError};
use crate::{FeeTier, TickSpacing, V3Pool};
use bindings::{ERC20Contract, V3PoolContract};
use ethers::contract::ContractError;
use ethers::{
    providers::Middleware,
    types::{Address, U256},
};
use futures::stream::{Collect, FuturesOrdered, FuturesUnordered, Map, StreamExt};
use futures::{Future, Stream};
use rug::Float;

pub struct Pool<M: Middleware> {
    pool: bindings::V3PoolContract<M>,
    tick_spacing: TickSpacing,
    token0: Address,
    token1: Address,
    token0_decimals: u8,
    token1_decimals: u8,
    fee: FeeTier,
}

impl<M: Middleware + 'static> Pool<M> {
    pub async fn new(
        pool: V3PoolContract<M>,
        middleware: std::sync::Arc<M>,
        fee: FeeTier,
    ) -> Result<Self, ContractError<M>> {
        let token0 = pool.token_0().call().await?;
        let token1 = pool.token_1().call().await?;

        let token0_decimals = ERC20Contract::new(token0, middleware.clone())
            .decimals()
            .call()
            .await?;

        let token1_decimals = ERC20Contract::new(token1, middleware.clone())
            .decimals()
            .call()
            .await?;

        Ok(Self {
            pool,
            tick_spacing: fee.as_spacing(),
            token0,
            token1,
            token0_decimals,
            token1_decimals,
            fee,
        })
    }

    async fn slot0(&self) -> Result<(U256, i32, u16, u16, u16, u8, bool), ContractError<M>> {
        Ok(self.pool.slot_0().await?)
    }
}

#[async_trait::async_trait]
impl<M: Middleware + 'static> V3Pool for Pool<M> {
    type Ticks = Pin<
        Box<
            dyn Stream<Item = Result<Float, V3PoolError<Self::BackendError>>>
                + Send,
        >,
    >;

    type BackendError = ContractError<M>;

    /// will error if the ticks arent n * spacing apart
    fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> Result<Self::Ticks, V3PoolError<Self::BackendError>> {
        let spacing = self.tick_spacing as i32;
        let mut down: bool = false;

        let differnce = starting - ending;
        let differnce = differnce.abs();
        let capactiy = differnce / spacing + 1;

        tracing::trace!("getting tick range");
        tracing::trace!(
            "starting: {:?}, ending: {:?}, spacing: {:?}",
            starting,
            ending,
            spacing
        );
        tracing::trace!("difference: {}, capacity: {}", differnce, capactiy);

        if differnce % (spacing as i32) != 0 {
            return Err(V3PoolError::BadTickRange(
                starting,
                ending,
                self.tick_spacing,
            ));
        }

        if starting > ending {
            down = true;
        }

        if starting == ending {
            return Ok(Box::pin(futures::stream::empty()));
        }

        let futs = {
            let mut futs = Vec::with_capacity(capactiy as usize);

            let mut current = starting;

            // we know this should happen because we check that the diff is a multiple of the spacing
            while current != ending {
                let pool = self.pool.clone();
                futs.push(async move {
                    let tick = pool
                        .ticks(current.into())
                        .await
                        .map_err(V3PoolError::backend_error)?;
        
                    Ok(Float::with_val(100, tick.1))
                });

                if down {
                    current = current.up(self.tick_spacing);
                } else {
                    current = current.down(self.tick_spacing);
                }
            }

            futs
        };

        if down {
            Ok(Box::pin(futs
                .into_iter()
                .collect::<FuturesUnordered<_>>()
                .map(|res| res.map(|x| Float::with_val(100, -x)))))
        } else {
            Ok(Box::pin(futs
                .into_iter()
                .collect::<FuturesUnordered<_>>()
                .map(|res| res.map(|x| Float::with_val(100, x)))))
        }
    }

    async fn current_liquidity(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let liquidity = self
            .pool
            .liquidity()
            .await
            .map_err(V3PoolError::backend_error)?;

        tracing::trace!("current liquidity: {}", liquidity);

        Ok(Float::with_val(100, liquidity))
    }

    async fn sqrt_price_x96(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let slot = self
            .pool
            .slot_0()
            .await
            .map_err(V3PoolError::backend_error)?;

        Ok(slot.0.into_float())
    }

    async fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>> {
        let tick = self
            .pool
            .ticks(tick.into())
            .await
            .map_err(V3PoolError::backend_error)?;

        Ok(Float::with_val(100, tick.1))
    }

    fn token0(&self) -> &Address {
        &self.token0
    }

    fn token1(&self) -> &Address {
        &self.token1
    }

    fn token0_decimals(&self) -> &u8 {
        &self.token0_decimals
    }

    fn token1_decimals(&self) -> &u8 {
        &self.token1_decimals
    }

    fn fee(&self) -> &FeeTier {
        &self.fee
    }

    fn address(&self) -> Address {
        self.pool.address()
    }
}
