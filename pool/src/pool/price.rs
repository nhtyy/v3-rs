use futures::TryStreamExt;
use rug::Float;

use crate::error::V3PoolError;
use crate::math::{Price, SqrtPrice, Tick};
use crate::V3Pool;
use crate::math::tick::{price_to_initializable_tick, tick_to_price};
use crate::types::deltas::Deltas;

#[async_trait::async_trait]
pub trait PriceExt: V3Pool {
    /// Returns the amount of token0 and token1 needed to move the pool price to the target price
    /// price_of_0_in_1 should not include the underlying nominal units
    async fn amounts_to_move_price(
        &self,
        new_price: Price,
    ) -> Result<Deltas<Self>, V3PoolError<Self::BackendError>> {
        let mut current_liquidity = self.current_liquidity().await?;

        tracing::debug!("current L: {:?}", current_liquidity);

        let mut current_sqrt_price = self.sqrt_price().await?;
        let target_sqrt_price: SqrtPrice = new_price.into();

        tracing::debug!("current sqrt price: {:?}", current_sqrt_price);
        tracing::debug!("target sqrt price: {:?}", target_sqrt_price);

        // at this point we dont know if were going up or down
        // but either way these are always the "lower" prices but 
        let starting_lower_tick =
            price_to_initializable_tick(current_sqrt_price.clone().into(), self.tick_spacing());

        let target_lower_tick: Tick =
            price_to_initializable_tick(target_sqrt_price.clone().into(), self.tick_spacing());

        tracing::trace!("starting_lower_tick {:?}", starting_lower_tick);
        tracing::trace!("target_lower_tick {:?}", target_lower_tick);

        let mut deltas = Deltas::new(self);
        let mut next_tick: Tick;
        let up: bool;

        // if were in the same tick we want to just move to the price and exit from there
        // if were going up lets move to the boundry, include that boundry and then move into the loop
        // if were going down lets move to the boundry, include that boundry and then move into the loop
        let ticks: Vec<Float> = if starting_lower_tick < target_lower_tick {
            tracing::debug!("current lower tick is less than target lower tick");
            tracing::debug!("were momving the price up");
            next_tick = starting_lower_tick.up(self.tick_spacing());
            up = true;

            // move the price to the next tick
            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );

            // get the tick range from the current tick to the target tick
            self.tick_range(next_tick, target_lower_tick.up(self.tick_spacing()))?
                .try_collect()
                .await?
        } else if starting_lower_tick > target_lower_tick {
            tracing::debug!("current lower tick is greater than target lower tick");
            tracing::debug!("were moving the price down");
            next_tick = starting_lower_tick;
            up = false;

            // move the price to the next tick
            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );

            // get the tick range from the current tick to the target tick
            self
                .tick_range(next_tick, target_lower_tick.down(self.tick_spacing()))?
                .try_collect()
                .await?
        } else {
            tracing::debug!("current lower tick is equal to target lower tick");

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price,
                target_sqrt_price.clone(),
            );

            return Ok(deltas);
        };

        tracing::trace!("starting tick loop");
        let mut ticks = ticks.into_iter();
        while let Some(delta) = ticks.next() {
            let current_tick = next_tick;

            current_liquidity += delta;
            if up {
                next_tick = current_tick.up(self.tick_spacing());
                current_sqrt_price = tick_to_price(current_tick).into();

                let next_tick_price: SqrtPrice = tick_to_price(next_tick).into();
                if next_tick_price > target_sqrt_price {
                    deltas.update(
                        current_liquidity.clone(),
                        current_sqrt_price.clone(),
                        target_sqrt_price.clone(),
                    );

                    tracing::trace!(?up, "exiting tick loop");

                    break;
                }
            } else {
                next_tick = current_tick.down(self.tick_spacing());
                current_sqrt_price = tick_to_price(current_tick).into();

                let next_tick_price: SqrtPrice = tick_to_price(next_tick).into();
                if next_tick_price < target_sqrt_price {
                    deltas.update(
                        current_liquidity.clone(),
                        current_sqrt_price.clone(),
                        target_sqrt_price.clone(),
                    );

                    tracing::trace!(?up, "exiting tick loop");

                    break;
                }
            }

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );
        }

        Ok(deltas)
    }

    async fn price_after() {
        todo!()
    }
}

impl<P: V3Pool> PriceExt for P {}