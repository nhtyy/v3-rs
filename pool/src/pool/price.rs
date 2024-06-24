use futures::TryStreamExt;

use crate::error::V3PoolError;
use crate::math::Tick;
use crate::V3Pool;
use crate::math::tick::{price_to_initializable_tick, price_to_tick, tick_to_price};
use crate::types::deltas::Deltas;

#[async_trait::async_trait]
pub trait PriceExt: V3Pool {
    /// Returns the amount of token0 and token1 needed to move the pool price to the target price
    /// price_of_0_in_1 should not include the underlying nominal units
    async fn amounts_to_move_price(
        &self,
        new_price: PoolPrice<Self>,
    ) -> Result<Deltas<Self>, V3PoolError<Self::BackendError>> {
        let mut current_liquidity = self.current_liquidity().await?;

        tracing::debug!("current L: {:?}", current_liquidity);

        let mut current_sqrt_price = self.sqrt_price().await?;
        let target_sqrt_price = new_price.into_pool_price_float().sqrt();

        tracing::debug!("current sqrt price: {:?}", current_sqrt_price);
        tracing::debug!("target sqrt price: {:?}", target_sqrt_price);

        let current_lower_tick =
            price_to_initializable_tick(current_sqrt_price.into(), self.tick_spacing());

        let target_lower_tick: Tick =
            price_to_initializable_tick(target_sqrt_price.into(), self.tick_spacing());

        tracing::trace!("current_lower_tick {:?}", current_lower_tick);
        tracing::trace!("target_lower_tick {:?}", target_lower_tick);

        let mut deltas = Deltas::new(self);
        let mut next_tick: Tick;
        let mut up: bool;

        let ticks = if current_lower_tick < target_lower_tick {
            tracing::debug!("current lower tick is less than target lower tick");
            up = true;

            // ending will be the lower tick of where the target price is
            // starting will be the upper tick of the current price is

            next_tick = current_lower_tick.up(self.tick_spacing());

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );

            self.tick_range(next_tick, target_lower_tick)
                .try_collect()
                .await
                .map_err(V3PoolError::backend_error)?
        } else if current_lower_tick > target_lower_tick {
            tracing::debug!("current lower tick is greater than target lower tick");
            up = false;
            
            // ending will be the upper tick of where the target price is
            // starting will be the lower tick of the current price

            next_tick = current_lower_tick;

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );

            self
                .tick_range(next_tick, target_lower_tick.up(self.tick_spacing()))
                .try_collect()
                .await
                .map_err(V3PoolError::backend_error)?
        } else {
            tracing::debug!("current lower tick is equal to target lower tick");
            vec![]
        };

        tracing::trace!(target = "n", "{:?}", ticks);

        let mut ticks = ticks.into_iter();
        while let Some(delta) = ticks.next() {
            let current_tick = next_tick;

            current_liquidity += delta;
            if up {
                next_tick = current_tick.up(self.tick_spacing());
            } else {
                next_tick = current_tick.down(self.tick_spacing());
            }

            current_sqrt_price = tick_to_price(current_tick).into();

            deltas.update(
                current_liquidity.clone(),
                current_sqrt_price.clone(),
                tick_to_price(next_tick).into(),
            );
        }

        deltas.update(
            current_liquidity.clone(),
            current_sqrt_price,
            target_sqrt_price.clone(),
        );

        Ok(deltas)
    }

    async fn price_after() {
        todo!()
    }

    async fn pool_price() {
        todo!()
    }
}