use std::fmt::Debug;

use crate::{
    error::V3PoolError,
    math::{SqrtPrice, Tick},
    traits::Batch,
    TokenAmount,
};
use alloy::{
    network::Network,
    primitives::{uint, Address, U256},
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
    P: Provider<T, N> + Clone,
    N: Network,
{
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            instance: PositionManager::PositionManagerInstance::new(address, provider),
        }
    }
}

impl<T, P, N> Manager<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub async fn all_positions(
        &self,
        owner: Address,
    ) -> Result<Vec<PositionsReturn>, alloy::contract::Error> {
        let balance = self.instance.balanceOf(owner).call().await?._0;

        let mut calls = Vec::with_capacity(balance.saturating_to());
        let mut i = U256::ZERO;
        while i < balance {
            calls.push(self.instance.tokenOfOwnerByIndex(owner, i));
            i += uint!(1_U256);
        }

        let ids = calls.batch().call().await?;
        Ok(ids
            .into_iter()
            .map(|id| self.instance.positions(id._0))
            .collect::<Vec<_>>()
            .batch()
            .call()
            .await?)
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
