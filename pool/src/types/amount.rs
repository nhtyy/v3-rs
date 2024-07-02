use std::ops::AddAssign;
use std::ops::Add;

use ethers::types::U256;
use rug::ops::Pow;
use rug::Float;

use super::Token;
use crate::traits::ApplyBps;
use crate::V3Pool;
use crate::traits::{IntoFloat, IntoU256, ConversionError};

#[derive(Clone)]
/// A token amount that belongs to a pool
/// 
/// There are helpers for converting between human readable amounts and scaled amounts
/// and display and debug implementations return the human readable amount
/// 
/// # Scaling Assumption
/// - PartialOrd & PartialEq with Self, [Float] and [U256] assuming theyre scaled
/// - Into<T> returns the scaled amounts
/// - Add and Sub with [U256] and [rug::Float] are assumed to be scaled
/// - PartialOrd & PartialEq with native types will scale the values
/// - Add and Sub with native types will scale the native types (u8, u16, u32, f16, f32) // todo
/// 
/// we purposefully dont implement Add and Sub with Self so were not repsonsible for checking
/// the amounts are actually the same type of token
/// 
/// this types makes no guarantees that the inner float is a valid amount
pub struct TokenAmount<'a, P> {
    pool: &'a P,
    token: Token,
    amount: Float,
}

impl<'a, P: V3Pool> TokenAmount<'a, P> {
    #[inline]
    pub fn decimals(&self) -> &u8 {
        match self.token {
            Token::Zero => self.pool.token0_decimals(),
            Token::One => self.pool.token1_decimals(),
        }
    }

    #[inline]
    pub fn token(&self) -> &Token {
        &self.token
    }

    #[inline]
    pub fn as_float(&self) -> &Float {
        &self.amount
    }

    #[inline]
    pub fn normalized_amount(&self) -> Float {
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

    /// Create a token amount from a scaled amount
    #[inline]
    pub unsafe fn from_scaled(pool: &'a P, token: Token, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount,
        }
    }

    /// Scale up a human readable amount to a scaled amount
    #[inline]
    fn scale_up(pool: &P, token: Token, amount: Float) -> Float {
        let exp = match token {
            Token::Zero => pool.token0_decimals(),
            Token::One => pool.token1_decimals(),
        };

        amount * Float::with_val(100, 10).pow(exp)
    }

    #[inline]
    fn scale_down(pool: &P, token: Token, amount: Float) -> Float {
        let exp = match token {
            Token::Zero => pool.token0_decimals(),
            Token::One => pool.token1_decimals(),
        };

        amount / Float::with_val(100, 10).pow(exp)
    }
}

impl<'a, P> ApplyBps for TokenAmount<'a, P> {
    fn apply_bps_down(&self, bps: u16) -> Self {
        Self {
            amount: self.amount.apply_bps_down(bps),
            token: self.token,
            pool: self.pool,
        }
    }

    fn apply_bps_up(&self, bps: u16) -> Self {
        Self {
            amount: self.amount.apply_bps_up(bps),
            token: self.token,
            pool: self.pool,
        }
    }
}

impl<'a, P> IntoU256 for TokenAmount<'a, P> {
    fn into_u256(&self) -> Result<U256, ConversionError> {
        self.amount.into_u256()
    }
}

impl<'a, P> From<TokenAmount<'a, P>> for Float {
    fn from(amount: TokenAmount<'a, P>) -> Self {
        amount.amount
    }
}

impl<'a, P> PartialOrd for TokenAmount<'a, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl<'a, P> PartialEq for TokenAmount<'a, P> {
    fn eq(&self, other: &Self) -> bool {
        self.amount.eq(&other.amount)
    }
}

impl<'a, P> Add<Float> for TokenAmount<'a, P> {
    type Output = Self;

    fn add(self, rhs: Float) -> Self::Output {
        Self {
            amount: self.amount + rhs,
            token: self.token,
            pool: self.pool,
        }
    }
}

impl<'a, P> Add<TokenAmount<'a, P>> for Float {
    type Output = Float;

    fn add(self, rhs: TokenAmount<'a, P>) -> Self::Output {
        self + rhs.amount
    }
}

impl<'a, P> Add<U256> for TokenAmount<'a, P> {
    type Output = Self;

    fn add(self, rhs: U256) -> Self::Output {
        Self {
            amount: self.amount + rhs.into_float(),
            token: self.token,
            pool: self.pool,
        }
    }
}

impl<'a, P> Add<TokenAmount<'a, P>> for U256 {
    type Output = TokenAmount<'a, P>;

    fn add(self, rhs: TokenAmount<'a, P>) -> Self::Output {
        TokenAmount {
            amount: self.into_float() + rhs.amount,
            token: rhs.token,
            pool: rhs.pool,
        }
    }
}

impl<'a, P> AddAssign<Float> for TokenAmount<'a, P> {
    fn add_assign(&mut self, rhs: Float) {
        self.amount += rhs;
    }
}

impl<'a, P> AddAssign<U256> for TokenAmount<'a, P> {
    fn add_assign(&mut self, rhs: U256) {
        self.amount += rhs.into_float();
    }
}

// Float and U256 implementations

impl<'a, P> PartialEq<Float> for TokenAmount<'a, P> {
    fn eq(&self, other: &Float) -> bool {
        self.amount.eq(other)
    }
}

impl<'a, P> PartialEq<TokenAmount<'a, P>> for Float {
    fn eq(&self, other: &TokenAmount<'a, P>) -> bool {
        self.eq(&other.amount)
    }
}

impl<'a, P> PartialOrd<Float> for TokenAmount<'a, P> {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(other)
    }
}

impl<'a, P> PartialOrd<TokenAmount<'a, P>> for Float {
    fn partial_cmp(&self, other: &TokenAmount<'a, P>) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.amount)
    }
}

impl<'a, P> PartialEq<U256> for TokenAmount<'a, P> {
    fn eq(&self, other: &U256) -> bool {
        self.amount.eq(&other.into_float())
    }
}

impl<'a, P> PartialEq<TokenAmount<'a, P>> for U256 {
    fn eq(&self, other: &TokenAmount<'a, P>) -> bool {
        self.into_float().eq(&other.amount)
    }
}

impl<'a, P> PartialOrd<U256> for TokenAmount<'a, P> {
    fn partial_cmp(&self, other: &U256) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.into_float())
    }
}

impl<'a, P> PartialOrd<TokenAmount<'a, P>> for U256 {
    fn partial_cmp(&self, other: &TokenAmount<'a, P>) -> Option<std::cmp::Ordering> {
        self.into_float().partial_cmp(&other.amount)
    }
}

impl_token_amount_cmp_eq_native!(
    TokenAmount
);

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
            .field("amount", &self.normalized_amount())
            .field("token", &self.token)
            .field("token0", &self.pool.token0())
            .field("token1", &self.pool.token1())
            .field("token0_decimals", &self.pool.token0_decimals())
            .field("token1_decimals", &self.pool.token1_decimals())
            .finish()
    }
}