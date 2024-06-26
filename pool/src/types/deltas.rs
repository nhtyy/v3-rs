use rug::Float;

use crate::math::swap::{token0_delta, token1_delta};
use crate::math::SqrtPrice;
use crate::types::amount::TokenAmount;
use crate::V3Pool;

/// The change change in token of a pool because of some operation
#[derive(Clone)]
pub struct Deltas<'a, P: V3Pool> {
    pool: &'a P,
    token0_amount: TokenAmount<'a, P>,
    token1_amount: TokenAmount<'a, P>,
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
        *self.token0_amount +=
            token0_delta(sqrt_price.clone(), target_price.clone(), liquidity.clone());

        *self.token1_amount += token1_delta(sqrt_price, target_price, liquidity);
    }

    /// Returns the additional amount of tokens needs to recieve the target amount
    ///
    /// Returns None if any amount is zero
    pub fn fee_amount(&self, fee_bp: u32) -> Option<TokenAmount<P>> {
        if self.token0_amount.is_zero() || self.token1_amount.is_zero() {
            return None;
        }

        // The positive amount is the amount coming into the pool
        match (
            self.token0_amount.is_sign_negative(),
            self.token1_amount.is_sign_negative(),
        ) {
            (true, false) => {
                let fee = Float::with_val(100, fee_bp);
                let fee = fee / 10000;
                let decay = 1 - fee;

                // Saftey: amounts came from valid token amounts
                unsafe {
                    Some(TokenAmount::from_scaled(
                        self.pool,
                        super::Token::One,
                        self.token1_amount.clone() / decay,
                    ))
                }
            }
            (false, true) => {
                let fee = Float::with_val(100, fee_bp);
                let fee = fee / 10000;
                let decay = 1 - fee;

                // Saftey: amounts came from valid token amounts
                unsafe {
                    Some(TokenAmount::from_scaled(
                        self.pool,
                        super::Token::Zero,
                        self.token0_amount.clone() / decay,
                    ))
                }
            }
            (_, _) => unreachable!("Got two non zero same sign deltas, this is a bug"),
        }
    }
}
