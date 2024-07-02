use crate::{
    ethers_pool::pool::Pool,
    math::{SqrtPrice, Tick},
};
use ethers::{
    contract::abigen,
    providers::Middleware,
    types::{Address, U256},
};
use futures::future::try_join_all;
use rug::Float;
use std::collections::HashMap;
use std::future::IntoFuture;

use crate::math::liquidity::{real_token0_from_l, real_token1_from_l};
use crate::math::tick::{price_to_tick, tick_to_price};
use crate::pool::V3Pool;

abigen!(
    PositionManager,
    "abi/PositionManager.json",
    derives(serde::Serialize, serde::Deserialize)
);

#[derive(Debug, Clone)]
pub struct Balances {
    pub token0: Address,
    pub token1: Address,
    pub amounts: HashMap<Address, Float>,
}

fn float_zero() -> Float {
    Float::with_val(18, 0)
}

impl<M: Middleware + 'static> PositionManager<M> {
    pub async fn all_positions(&self, owner: Address) -> anyhow::Result<Vec<PositionsReturn>> {
        // Get the number of postions this user has
        let balance: u64 = self.balance_of(owner).await?.as_u64();
        let mut id_futs: Vec<_> = Vec::new();

        // Get all the positon ids
        for i in 0..balance {
            id_futs.push(
                self.token_of_owner_by_index(owner, U256::from(i))
                    .into_future(),
            );
        }

        let pos_futs = try_join_all(id_futs)
            .await?
            .into_iter()
            .map(|id| async move { self.positions(id).await.map(|pos| pos.into()) });

        Ok(try_join_all(pos_futs).await?)
    }

    pub async fn total_positions_balance<P: V3Pool>(
        &self,
        pool: &P,
        owner: Address,
    ) -> anyhow::Result<Balances> {
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
            token0: *pool.token0(),
            token1: *pool.token1(),
            amounts: vec![
                (*pool.token0(), float_zero()),
                (*pool.token1(), float_zero()),
            ]
            .into_iter()
            .collect(),
        };

        Ok(positions
            .into_iter()
            .map(|pos| pos.token_balances(sqrt_price.clone()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(init, |acc, balance| Balances {
                amounts: acc
                    .amounts
                    .into_iter()
                    .map(|(k, v)| (k, v + balance.amounts.get(&k).unwrap_or(&float_zero())))
                    .collect(),
                ..acc
            }))
    }
}

impl PositionsReturn {
    pub fn token_balances(
        &self,
        sqrt_price: SqrtPrice,
    ) -> anyhow::Result<Balances> {
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

        Ok(Balances {
            token0: self.token_0,
            token1: self.token_1,
            amounts: vec![(self.token_0, token0_amount), (self.token_1, token1_amount)]
                .into_iter()
                .collect(),
        })
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
