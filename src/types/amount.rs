use std::ops::Add;
use std::ops::AddAssign;

use alloy::primitives::Address;
use alloy::primitives::U256;
use rug::ops::Pow;
use rug::Float;

use crate::TokenIdx;
use crate::traits::ApplyBps;
use crate::traits::{ConversionError, IntoFloat, IntoU256};
use crate::V3Pool;

/// A token amount that belongs to a pool
///
/// There are helpers for converting between human readable amounts and scaled amounts
/// and display and debug implementations return the human readable amount
///
/// # Scaling Assumption
/// - PartialOrd & PartialEq with Self, [Float] and [U256] assuming theyre scaled
/// - Into<T> returns the scaled amounts
/// - Add with [U256] and [rug::Float] are assumed to be scaled
///
/// - PartialOrd & PartialEq with native types will scale the the values (u8, u16, u32, f16, f32)
/// - Add and Sub with native types will scale the native types (u8, u16, u32, f16, f32)
///
/// No guarntees the amount is in range
pub struct TokenAmount<'a, P> {
    pool: &'a P,
    token: TokenIdx,
    amount: Float,
}

impl<'a, P> Clone for TokenAmount<'a, P> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool,
            token: self.token,
            amount: self.amount.clone(),
        }
    }
}

impl<'a, P: V3Pool> TokenAmount<'a, P> {
    #[inline]
    pub fn decimals(&self) -> &u8 {
        match self.token {
            TokenIdx::Zero => self.pool.token0_decimals(),
            TokenIdx::One => self.pool.token1_decimals(),
        }
    }

    #[inline]
    pub fn token(&self) -> &TokenIdx {
        &self.token
    }

    #[inline]
    pub fn token_address(&self) -> &Address {
        match self.token {
            TokenIdx::Zero => self.pool.token0(),
            TokenIdx::One => self.pool.token1(),
        }
    }

    #[inline]
    pub fn as_float(&self) -> &Float {
        &self.amount
    }

    #[inline]
    pub fn human_readable(&self) -> Float {
        Self::scale_down(self.pool, self.token, self.amount.clone())
    }

    #[inline]
    /// Create a new TokenAmount with a zero amount
    pub fn zero(pool: &'a P, token: TokenIdx) -> Self {
        Self {
            pool,
            token,
            amount: Float::with_val(100, 0),
        }
    }

    #[inline]
    /// Create a token amount from a human readable amount
    pub fn from_human_readable(pool: &'a P, token: TokenIdx, amount: f64) -> Self {
        Self {
            pool,
            token,
            amount: Self::scale_up(pool, token, amount),
        }
    }

    /// Create a token amount from a scaled amount
    #[inline]
    pub fn from_scaled(pool: &'a P, token: TokenIdx, amount: Float) -> Self {
        Self {
            pool,
            token,
            amount,
        }
    }

    /// Scale up a human readable amount to a scaled amount
    #[inline]
    fn scale_up(pool: &P, token: TokenIdx, amount: f64) -> Float {
        let exp = match token {
            TokenIdx::Zero => pool.token0_decimals(),
            TokenIdx::One => pool.token1_decimals(),
        };

        amount * Float::with_val(100, 10).pow(exp)
    }

    #[inline]
    fn scale_down(pool: &P, token: TokenIdx, amount: Float) -> Float {
        let exp = match token {
            TokenIdx::Zero => pool.token0_decimals(),
            TokenIdx::One => pool.token1_decimals(),
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
    type Output = TokenAmount<'a, P>;

    fn add(self, rhs: TokenAmount<'a, P>) -> Self::Output {
        rhs + self
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

impl<'a, P> AddAssign<&Float> for TokenAmount<'a, P> {
    fn add_assign(&mut self, rhs: &Float) {
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

pub trait IntoTokenAmount<'a, P: V3Pool> {
    fn into_token_amount(self, pool: &'a P, token: TokenIdx) -> TokenAmount<'a, P>;
}

impl<'a, P: V3Pool> IntoTokenAmount<'a, P> for TokenAmount<'a, P> {
    #[inline]
    fn into_token_amount(self, _pool: &'a P, _token: TokenIdx) -> TokenAmount<'a, P> {
        self
    }
}

impl<'a, P: V3Pool> IntoTokenAmount<'a, P> for Float {
    fn into_token_amount(self, pool: &'a P, token: TokenIdx) -> TokenAmount<'a, P> {
        TokenAmount::from_scaled(pool, token, self)
    }
}

impl<'a, P: V3Pool> IntoTokenAmount<'a, P> for U256 {
    fn into_token_amount(self, pool: &'a P, token: TokenIdx) -> TokenAmount<'a, P> {
        TokenAmount::from_scaled(pool, token, self.into_float())
    }
}

// Implementations for native types that scales them during ops
impl_token_amount_cmp_eq_native!(TokenAmount);

impl<'a, P: V3Pool> std::fmt::Display for TokenAmount<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.human_readable()
        )
    }
}

impl<'a, P: V3Pool> std::fmt::Debug for TokenAmount<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self.token {
            TokenIdx::Zero => self.pool.token0(),
            TokenIdx::One => self.pool.token1(),
        };

        f.debug_struct("TokenAmount")
            .field("raw", self.as_float())
            .field("human_readable", &self.human_readable())
            .field("token", token)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::TokenAmount;
    use crate::{FeeTier, TokenIdx};
    use alloy::primitives::Address;
    use alloy::primitives::U256;
    use rug::ops::Pow;
    use rug::Float;

    use crate::types::tests::MockPool;

    #[test]
    fn test_eq_float() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = TokenAmount::from_scaled(&pool, TokenIdx::Zero, Float::with_val(100, 100));
        let amount2 = Float::with_val(100, 100);

        assert_eq!(amount, amount2);
    }

    #[test]
    fn test_eq_u256() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = TokenAmount::from_scaled(&pool, TokenIdx::Zero, Float::with_val(100, 100));
        let amount2 = U256::from(100);

        assert_eq!(amount, amount2);
    }

    #[test]
    fn test_cmp_float() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = TokenAmount::from_scaled(&pool, TokenIdx::Zero, Float::with_val(100, 100));
        let amount2 = Float::with_val(100, 99);

        assert!(amount > amount2);
        assert!(amount2 < amount);
    }

    #[test]
    fn test_cmp_u256() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = TokenAmount::from_scaled(&pool, TokenIdx::Zero, Float::with_val(100, 100));
        let amount2 = U256::from(99);

        assert!(amount > amount2);
        assert!(amount2 < amount);
    }

    #[test]
    fn test_eq_native_u8() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = 100_u8;
        let token_amount = TokenAmount::from_human_readable(&pool, TokenIdx::Zero, amount.into());

        assert_eq!(amount, token_amount);
    }

    #[test]
    fn test_eq_native_f64() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = 100.0;
        let token_amount = TokenAmount::from_human_readable(&pool, TokenIdx::Zero, amount);

        assert_eq!(amount, token_amount);
    }

    #[test]
    fn test_returns_scaled_version() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let scalar = Float::with_val(100, 10).pow(18);

        let amount = TokenAmount::from_human_readable(&pool, TokenIdx::Zero, 100.0);
        let scaled = amount.as_float();

        assert_eq!(*scaled, Float::with_val(100, 100) * scalar);
    }

    #[test]
    fn test_add_float() {
        let pool = MockPool {
            token0: Address::ZERO,
            token1: Address::ZERO,
            token0_decimals: 18,
            token1_decimals: 18,
            fee: FeeTier::Mid,
        };

        let amount = TokenAmount::from_scaled(&pool, TokenIdx::Zero, Float::with_val(100, 100));
        let result = amount + Float::with_val(100, 100);

        assert_eq!(result, Float::with_val(10, 200));
    }
}
