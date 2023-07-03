use crate::pool::{FeeTier, SwapMath, Tick, TickSpacing, V3Pool, V3PoolError};
use bindings::{ERC20Contract, V3PoolContract};
use ethers::contract::ContractError;
use ethers::{
    providers::Middleware,
    types::{Address, U256},
};
use rug::{float::ParseFloatError, ops::Pow, Float};

/// Blocks the current thread
/// You should spawn a new thread for this
pub struct Pool<M: Middleware> {
    rt: tokio::runtime::Runtime,
    pool: bindings::V3PoolContract<M>,
    tick_spacing: TickSpacing,
    token0: Address,
    token1: Address,
    token0_decimals: u8,
    token1_decimals: u8,
    fee: FeeTier,
}

impl<M: Middleware + 'static> Pool<M> {
    pub(super) fn new(
        pool: V3PoolContract<M>,
        middleware: std::sync::Arc<M>,
        rt: tokio::runtime::Runtime,
        fee: FeeTier,
    ) -> Result<Self, V3PoolError<ContractError<M>>> {
        let (token0, token1, token0_decimals, token1_decimals) = rt.block_on(async {
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

            let result: Result<_, V3PoolError<ContractError<M>>> =
                Ok((token0, token1, token0_decimals, token1_decimals));

            result
        })?;

        Ok(Self {
            rt,
            pool,
            tick_spacing: fee.as_spacing(),
            token0,
            token1,
            token0_decimals,
            token1_decimals,
            fee,
        })
    }

    async fn fut_tick(&self, tick: Tick) -> Result<i128, ContractError<M>> {
        let tick = self.pool.ticks(tick as i32).await?;

        Ok(tick.1)
    }
}

impl<M: Middleware + 'static> V3Pool for Pool<M> {
    type Ticks = Vec<Float>;
    type BackendError = ContractError<M>;

    /// will error if the ticks arent n * spacing apart
    fn tick_range(
        &self,
        starting: Tick,
        ending: Tick,
    ) -> Result<Self::Ticks, V3PoolError<Self::BackendError>> {
        let mut spacing = self.tick_spacing as i32;
        let mut down: bool = false;

        let differnce = starting - ending;
        let capactiy = differnce / spacing + 1;

        if differnce % (spacing as i32) != 0 {
            return Err(V3PoolError::BadRange(starting, ending, spacing));
        }

        let res = self.rt.block_on(async {
            if starting > ending {
                down = true;
                spacing = -spacing;
            }

            if starting == ending {
                return Ok(vec![]);
            }

            let futs = {
                let mut futs = Vec::with_capacity(capactiy as usize);
                let mut current_tick = starting;

                // we know this should happen because we check that the diff is a multiple of the spacing
                while current_tick != ending {
                    let tick = self.fut_tick(current_tick);
                    futs.push(tick);
                    current_tick += spacing as i32;
                }

                futs
            };

            futures::future::try_join_all(futs).await
        });

        if down {
            Ok(res?.into_iter().map(|x| Float::with_val(100, -x)).collect())
        } else {
            Ok(res?.into_iter().map(|x| Float::with_val(100, x)).collect())
        }
    }

    fn tick_spacing(&self) -> TickSpacing {
        self.tick_spacing
    }

    fn fee(&self) -> FeeTier {
        self.fee
    }

    fn current_liquidity(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        self.rt.block_on(async {
            let liquidity = self.pool.liquidity().await?;

            Ok(Float::with_val(100, liquidity))
        })
    }

    fn token0(&self) -> Address {
        self.token0
    }

    fn token1(&self) -> Address {
        self.token1
    }

    fn sqrt_price_x96(&self) -> Result<U256, V3PoolError<Self::BackendError>> {
        self.rt.block_on(async {
            let slot = self.pool.slot_0().await?;

            Ok(slot.0)
        })
    }

    fn tick(&self, tick: Tick) -> Result<Float, V3PoolError<Self::BackendError>> {
        self.rt.block_on(async {
            let tick = self.pool.ticks(tick as i32).await?;

            Ok(Float::with_val(100, tick.1))
        })
    }

    fn token0_decimals(&self) -> u8 {
        self.token0_decimals
    }

    fn token1_decimals(&self) -> u8 {
        self.token1_decimals
    }
}

impl<M: Middleware> From<ContractError<M>> for V3PoolError<ContractError<M>> {
    fn from(err: ContractError<M>) -> Self {
        V3PoolError::BackendError(err)
    }
}
