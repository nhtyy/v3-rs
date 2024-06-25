use std::ops::Deref;
use std::ops::DerefMut;

use rug::ops::Pow;
use rug::Float;

use crate::Token;
use crate::V3Pool;

#[derive(Clone)]
/// A token amount that belongs to a pool
/// 
/// There are helpers for converting between human readable amounts and scaled amounts
/// and display and debug implementations return the human readable amount
pub struct TokenAmount<'a, P: V3Pool> {
    pool: &'a P,
    token: Token,
    amount: Float,
}

impl<'a, P: V3Pool> TokenAmount<'a, P> {
    #[inline]
    pub fn token(&self) -> &Token {
        &self.token
    }

    #[inline]
    pub fn amount(&self) -> &Float {
        &self.amount
    }

    #[inline]
    pub fn human_readable_amount(&self) -> Float {
        Self::scale_down(self.pool, self.token, self.amount.clone())
    }

    #[inline]
    /// Create a new TokenAmount with a zero amount
    pub fn zero(pool: &'a P, token: Token) -> Self {
        Self {
            pool,
            token,
            amount: Float::with_val(100, 0),
        }
    }

    #[inline]
    /// Create a token amount from a human readable amount
    pub fn from_amount(pool: &'a P, token: Token, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount: Self::scale_up(pool, token, amount),
        }
    }

    #[inline]
    /// Create a token amount from a scaled amount
    pub unsafe fn from_scaled(pool: &'a P, token: Token, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount,
        }
    }

    /// Scale up a human readable amount to a scaled amount
    fn scale_up(pool: &P, token: Token, amount: Float) -> Float {
        let exp = match token {
            Token::Zero => pool.token0_decimals(),
            Token::One => pool.token1_decimals(),
        };

        amount * Float::with_val(100, 10).pow(exp)
    }

    fn scale_down(pool: &P, token: Token, amount: Float) -> Float {
        let exp = match token {
            Token::Zero => pool.token0_decimals(),
            Token::One => pool.token1_decimals(),
        };

        amount / Float::with_val(100, 10).pow(exp)
    }
}

impl<'a, P: V3Pool> std::fmt::Display for TokenAmount<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Self::scale_down(self.pool, self.token, self.amount.clone())
        )
    }
}

impl<'a, P: V3Pool> std::fmt::Debug for TokenAmount<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenAmount")
            .field("amount", &self.amount)
            .field("token", &self.token)
            .field("token0", &self.pool.token0())
            .field("token1", &self.pool.token1())
            .field("token0_decimals", &self.pool.token0_decimals())
            .field("token1_decimals", &self.pool.token1_decimals())
            .finish()
    }
}

impl<'a, P: V3Pool> Deref for TokenAmount<'a, P> {
    type Target = Float;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.amount
    }
}

impl<'a, P: V3Pool> DerefMut for TokenAmount<'a, P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amount
    }
}
