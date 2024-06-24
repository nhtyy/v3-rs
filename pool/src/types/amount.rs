use std::ops::Deref;
use std::ops::DerefMut;

use rug::ops::Pow;
use rug::Float;

use crate::Token;
use crate::V3Pool;

pub struct TokenAmount<'a, P: V3Pool> {
    pool: &'a P,
    token: Token,
    amount: Float,
}

impl<'a, P: V3Pool> TokenAmount<'a, P> {
    #[inline]
    pub fn zero(pool: &'a P, token: Token) -> Self {
        Self {
            pool,
            token,
            amount: Float::with_val(100, 0),
        }
    }

    #[inline]
    pub fn from_human_readable(pool: &'a P, token: Token, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount: Self::scale_up(pool, token, amount),
        }
    }

    #[inline]
    pub fn from_scaled(pool: &'a P, token: Token, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount,
        }
    }

    pub fn scale_up(pool: &P, token: Token, amount: Float) -> Float {
        let exp = match token {
            Token::Zero => pool.token0_decimals(),
            Token::One => pool.token1_decimals(),
        };

        amount * Float::with_val(100, 10).pow(exp)
    }

    pub fn scale_down(pool: &P, token: Token, amount: Float) -> Float {
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
