use std::fmt::Debug;

use crate::{
    error::V3PoolError,
    math::{SqrtPrice, Tick},
    TokenAmount,
};
use alloy::{
    contract::{MultiCall, MultiCallError},
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    transports::Transport,
};
use rug::Float;

use crate::math::liquidity::{real_token0_from_l, real_token1_from_l};
use crate::math::tick::{price_to_tick, tick_to_price};
use crate::pool::V3Pool;

alloy::sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    interface PositionManager {
        function positions(uint256 tokenId)
            external
            view
            returns (
                uint96 nonce,
                address operator,
                address token0,
                address token1,
                uint24 fee,
                int24 tickLower,
                int24 tickUpper,
                uint128 liquidity,
                uint256 feeGrowthInside0LastX128,
                uint256 feeGrowthInside1LastX128,
                uint128 tokensOwed0,
                uint128 tokensOwed1
            );

        function balanceOf(address owner) external view returns (uint256);

        function tokenOfOwnerByIndex(address owner, uint256 idx) returns (uint256);
    }
}

pub use PositionManager::positionsReturn as PositionsReturn;

/// A read only wrapper around the NFT position manager contract
pub struct Manager<T, P, N> {
    instance: PositionManager::PositionManagerInstance<T, P, N>,
}

#[derive(Clone)]
pub struct Balances<'a, P: V3Pool> {
    tokens: [TokenAmount<'a, P>; 2],
}

impl<'a, P: V3Pool> Debug for Balances<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Balances")
            .field("token0", &self.tokens[0])
            .field("token1", &self.tokens[1])
            .finish()
    }
}

impl<'a, P: V3Pool> Balances<'a, P> {
    pub fn new(token0: TokenAmount<'a, P>, token1: TokenAmount<'a, P>) -> Self {
        Self {
            tokens: [token0, token1],
        }
    }

    pub fn token0(&self) -> &TokenAmount<'a, P> {
        &self.tokens[0]
    }

    pub fn token1(&self) -> &TokenAmount<'a, P> {
        &self.tokens[1]
    }
}

impl<T, P, N> Manager<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub const fn new(address: Address, provider: P) -> Self {
        Self {
            instance: PositionManager::PositionManagerInstance::new(address, provider),
        }
    }

    pub async fn all_positions(
        &self,
        owner: Address,
    ) -> Result<Vec<PositionsReturn>, alloy::contract::Error> {
        let balance = self.instance.balanceOf(owner).call().await?;

        // todo branch if not multicall chain?
        async fn all_positions_multicall<T, P, N>(
            this: &Manager<T, P, N>,
            balance: U256,
            owner: Address,
        ) -> Result<Vec<PositionsReturn>, alloy::contract::MultiCallError>
        where
            T: Transport + Clone,
            P: Provider<T, N>,
            N: Network,
        {
            let multicall = MultiCall::new_checked(this.instance.provider()).await?;
            let mut aggregate = multicall.aggregate();

            let mut i = U256::ZERO;
            while i < balance {
                aggregate.add_call(this.instance.tokenOfOwnerByIndex(owner, i));
                i += U256::from(1u8);
            }

            let ids = aggregate.call_consume().await?;
            let mut aggregate = multicall.aggregate();
            aggregate.extend(ids.into_iter().map(|id| this.instance.positions(id._0)));

            aggregate.call_consume().await
        }

        match all_positions_multicall(self, balance._0, owner).await {
            Ok(positions) => Ok(positions),
            Err(MultiCallError::ContractError(e)) => Err(e),
            Err(MultiCallError::ChainNotSupported) => {
                panic!("Tried to get all positions on a chain that doesnt have multicall")
            }
            Err(MultiCallError::MissingTargetAddress) => {
                unreachable!("All calls should have a target address, this is a bug")
            }
        }
    }

    pub async fn total_positions_balance<'p, Pool>(
        &self,
        pool: &'p Pool,
        owner: Address,
    ) -> Result<Balances<'p, Pool>, V3PoolError<Pool::BackendError>>
    where
        Pool: V3Pool<BackendError = alloy::contract::Error>,
    {
        let positions = self
            .all_positions(owner)
            .await
            .map_err(V3PoolError::backend_error)?
            .into_iter()
            .filter(|pos| {
                pos.token0 == *pool.token0()
                    && pos.token1 == *pool.token1()
                    && pos.fee == pool.fee().as_scaled_bp()
            })
            .collect::<Vec<_>>();

        tracing::debug!("Positions for {:#?}:\n {:#?}", owner, positions);

        let sqrt_price = pool.sqrt_price().await?;

        let init = Balances {
            tokens: [
                TokenAmount::zero(pool, crate::TokenIdx::Zero),
                TokenAmount::zero(pool, crate::TokenIdx::One),
            ],
        };

        Ok(positions
            .iter()
            .map(|pos| pos.token_balances(pool, sqrt_price.clone()))
            .collect::<Vec<_>>()
            .into_iter()
            .fold(init, |mut acc, balance| {
                acc.tokens[0] += balance.token0().as_float();
                acc.tokens[1] += balance.token1().as_float();

                acc
            }))
    }
}

impl PositionsReturn {
    pub fn token_balances<'a, P: V3Pool>(
        &self,
        pool: &'a P,
        sqrt_price: SqrtPrice,
    ) -> Balances<'a, P> {
        let current_tick = price_to_tick(sqrt_price.clone().into());

        // saftey: comes from pool
        let tick_upper = unsafe { Tick::new_unchecked(self.tickUpper.as_i32()) };
        let tick_lower = unsafe { Tick::new_unchecked(self.tickLower.as_i32()) };

        let upper_price: SqrtPrice = tick_to_price(tick_upper).into();
        let lower_price: SqrtPrice = tick_to_price(tick_lower).into();

        let l = Float::with_val(100, self.liquidity);

        let token0_amount = if current_tick >= tick_lower && current_tick < tick_upper {
            real_token0_from_l(sqrt_price.clone(), upper_price.clone(), l.clone())
        } else if current_tick < tick_lower {
            real_token0_from_l(lower_price.clone(), upper_price.clone(), l.clone())
        } else {
            Float::with_val(18, 0)
        };

        let token1_amount = if current_tick >= tick_lower && current_tick < tick_upper {
            real_token1_from_l(sqrt_price, lower_price, l)
        } else if current_tick > tick_upper {
            real_token1_from_l(upper_price, lower_price, l)
        } else {
            Float::with_val(18, 0)
        };

        Balances::new(
            TokenAmount::from_scaled(pool, crate::TokenIdx::Zero, token0_amount),
            TokenAmount::from_scaled(pool, crate::TokenIdx::One, token1_amount),
        )
    }
}
