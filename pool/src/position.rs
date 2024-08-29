use crate::{math::{SqrtPrice, Tick}, TokenAmount};
use ethers::{
    contract::{abigen, Multicall, MULTICALL_ADDRESS},
    providers::Middleware,
    types::{Address, U256},
};
use futures::future::try_join_all;
use rug::Float;
use std::future::IntoFuture;

use crate::math::liquidity::{real_token0_from_l, real_token1_from_l};
use crate::math::tick::{price_to_tick, tick_to_price};
use crate::pool::V3Pool;
use ethers::contract::Contract;

abigen!(
    PositionManager,
    "abi/PositionManager.json",
    derives(serde::Serialize, serde::Deserialize)
);

#[derive(Debug, Clone)]
pub struct Balances<'a, P: V3Pool> {
    tokens: [TokenAmount<'a, P>; 2]
}

impl<'a ,P: V3Pool> Balances<'a, P> {
    pub fn new(token0: TokenAmount<'a, P>, token1: TokenAmount<'a, P>) -> Self {
        Self {
            tokens: [token0, token1]
        }
    }

    pub fn token0(&self) -> &TokenAmount<'a, P> {
        &self.tokens[0]
    }

    pub fn token1(&self) -> &TokenAmount<'a, P> {
        &self.tokens[1]
    }
}



fn float_zero() -> Float {
    Float::with_val(18, 0)
}

impl<M: Middleware + 'static> PositionManager<M> {
    pub async fn all_positions(&self, owner: Address) -> anyhow::Result<Vec<PositionsReturn>> {
        // Get the number of postions this user has
        let balance = self.balance_of(owner).await?.as_u64();

        let mut multicall: Multicall<M> = Multicall::new(self.client().clone(), Some(MULTICALL_ADDRESS)).await?;

        // Get all the positon ids
        for i in 0..balance {
            multicall.add_call(
                self.token_of_owner_by_index(owner, U256::from(i)),
                false,
            );
        }

        let ids = multicall.call_array::<U256>().await?;

        multicall.clear_calls();

        let pos_futs = ids
            .into_iter()
            .map(|id| async move { self.positions(id).await.map(|pos| pos.into()) });

        Ok(try_join_all(pos_futs).await?)
    }

    pub async fn total_positions_balance<'a, P: V3Pool>(
        &'a self,
        pool: &'a P,
        owner: Address,
    ) -> anyhow::Result<Balances<'a, P>> {
        let positions = self
            .all_positions(owner)
            .await?
            .into_iter()
            .filter(|pos| {
                pos.token_0 == *pool.token0()
                    && pos.token_1 == *pool.token1()
                    && pos.fee == pool.fee().as_scaled_bp()
            })
            .collect::<Vec<_>>();

        tracing::debug!("Positions for {:#?}:\n {:#?}", owner, positions);

        let sqrt_price = pool.sqrt_price().await?;

        let init = Balances {
            tokens: [
                TokenAmount::zero(pool, crate::Token::Zero),
                TokenAmount::zero(pool, crate::Token::One),
            ],
        };

        Ok(positions
            .iter()
            .map(|pos| pos.token_balances(pool, sqrt_price.clone()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(init, |mut acc, balance| {
                acc.tokens[0] += balance.token0().as_float();
                acc.tokens[1] += balance.token1().as_float();

                acc
            }))
    }
}

impl PositionsReturn {
    pub fn token_balances<'a, P: V3Pool>(&'a self, pool: &'a P, sqrt_price: SqrtPrice) -> anyhow::Result<Balances<'a, P>> {
        let current_tick = price_to_tick(sqrt_price.clone().into());

        // saftey: comes from pool
        let tick_upper = unsafe { Tick::new_unchecked(self.tick_upper) };
        let tick_lower = unsafe { Tick::new_unchecked(self.tick_lower) };

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

        Ok(Balances::new(
            TokenAmount::from_scaled(pool, crate::Token::Zero, token0_amount),
            TokenAmount::from_scaled(pool, crate::Token::One, token1_amount)
        ))
    }
}

/// Not sure why this is needed
impl
    From<(
        u128,
        ethers::core::types::Address,
        ethers::core::types::Address,
        ethers::core::types::Address,
        u32,
        i32,
        i32,
        u128,
        ethers::core::types::U256,
        ethers::core::types::U256,
        u128,
        u128,
    )> for PositionsReturn
{
    fn from(
        (_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11): (
            u128,
            ethers::core::types::Address,
            ethers::core::types::Address,
            ethers::core::types::Address,
            u32,
            i32,
            i32,
            u128,
            ethers::core::types::U256,
            ethers::core::types::U256,
            u128,
            u128,
        ),
    ) -> Self {
        Self {
            nonce: _0,
            operator: _1,
            token_0: _2,
            token_1: _3,
            fee: _4,
            tick_lower: _5,
            tick_upper: _6,
            liquidity: _7,
            fee_growth_inside_0_last_x128: _8,
            fee_growth_inside_1_last_x128: _9,
            tokens_owed_0: _10,
            tokens_owed_1: _11,
        }
    }
}
