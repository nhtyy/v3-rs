use std::fmt::Formatter;

use rug::Float;

use crate::math::swap::{token0_delta, token1_delta};
use crate::math::SqrtPrice;
use crate::types::amount::TokenAmount;
use crate::V3Pool;

/// The change change in token of a pool because of some operation
#[derive(Clone)]
pub struct Deltas<'a, P: V3Pool> {
    pool: &'a P,
    pub token0_amount: TokenAmount<'a, P>,
    pub token1_amount: TokenAmount<'a, P>,
}

impl<'a, P: V3Pool> std::fmt::Debug for Deltas<'a, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deltas")
            .field("token0_amount", &self.token0_amount)
            .field("token1_amount", &self.token1_amount)
            .finish()
    }
}

impl<'a, P: V3Pool> Deltas<'a, P> {
    pub fn token0_amount(&self) -> &TokenAmount<P> {
        &self.token0_amount
    }

    pub fn token1_amount(&self) -> &TokenAmount<P> {
        &self.token1_amount
    }

    pub fn new(pool: &'a P) -> Self {
        Self {
            pool,
            token0_amount: TokenAmount::zero(pool, super::Token::Zero),
            token1_amount: TokenAmount::zero(pool, super::Token::One),
        }
    }

    pub fn update(&mut self, liquidity: Float, sqrt_price: SqrtPrice, target_price: SqrtPrice) {
        tracing::trace!(
            "updating deltas with liquidity: {}, sqrt_price: {}, target_price: {}",
            liquidity,
            sqrt_price,
            target_price
        );

        self.token0_amount +=
            token0_delta(sqrt_price.clone(), target_price.clone(), liquidity.clone());

        self.token1_amount += token1_delta(sqrt_price, target_price, liquidity);
    }

    /// Returns the additional amount of tokens you would need to add to your swap to cover the fee
    ///
    /// Returns None if any amount is zero
    pub fn fee_amount(&self) -> TokenAmount<P> {
        let token1_amount = self.token1_amount.as_float();
        let token0_amount = self.token0_amount.as_float();

        if token0_amount.is_zero() || token1_amount.is_zero() {
            tracing::warn!("got a zero token trade");
            return TokenAmount::zero(self.pool, super::Token::Zero);
        }

        let fee = Float::with_val(100, self.pool.fee().to::<u32>());
        let fee = fee / 1e6;
        let decay = 1 - fee;

        // The positive amount is the amount coming into the pool
        match (
            token0_amount.is_sign_negative(),
            token1_amount.is_sign_negative(),
        ) {
            (true, false) => TokenAmount::from_scaled(
                self.pool,
                super::Token::One,
                token1_amount.clone() / decay,
            ),
            (false, true) => TokenAmount::from_scaled(
                self.pool,
                super::Token::Zero,
                token0_amount.clone() / decay,
            ),
            (_, _) => unreachable!("Got two non zero same sign deltas, this is a bug"),
        }
    }
}
