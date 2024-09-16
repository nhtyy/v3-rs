use std::str::FromStr;

use alloy::primitives::{Signed, Uint, U256};
use lazy_static::lazy_static;
use rug::Float;
use rug::ops::Pow;

use crate::traits::IntoU256;
use crate::TickSpacing;

lazy_static! {
    pub static ref MAX_PRICE: Float = Float::with_val(100, 1.0001).pow(887272);
    pub static ref MIN_PRICE: Float = Float::with_val(100, 1.0001).pow(-887272);

    pub static ref MAX_SQRT_PRICE: Float = MAX_PRICE.clone().sqrt();
    pub static ref MIN_SQRT_PRICE: Float = MIN_PRICE.clone().sqrt();
}

#[derive(Clone, Debug)]
/// An error that occurs when a value is out of bounds for a type
/// checked at compile time
pub struct BoundsError(&'static str, String);

impl std::fmt::Display for BoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bounds error on type {}, given value: {}", self.0, self.1)
    }
}

impl std::error::Error for BoundsError {}

impl_wrappers! {
    /// A *valid* tick in the range [-887272, 887272]
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
    pub struct Tick(i32);

    /// A raw in-range pool price
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Price(Float);

    /// A raw in range sqrt pool sqrt price
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct SqrtPrice(Float);
}

impl Tick {
    /// Creates a new *valid* tick from an i32
    /// 
    /// # Errors:
    /// - if the tick is less than -887272
    /// - if the tick is greater than 887272
    pub fn new(tick: i32) -> Result<Self, BoundsError> {
        if tick.abs() <= 887272 {
            Ok(Self(tick))
        } else {
            Err(BoundsError("Tick", tick.to_string()))
        }
    }

    /// Creates a new tick from an i32 without checking if it is valid
    /// useful for when youre reading directly from the pool
    pub unsafe fn new_unchecked(tick: i32) -> Self {
        Self(tick)
    }

    /// Returns the next initializable tick
    /// 
    /// # Panics: 
    /// - if the next tick is out of bounds
    pub fn up(self, spacing: TickSpacing) -> Self {
        let spacing = spacing as i32;
        let scalar = self.0 / spacing + 1;
        let next_tick = scalar * spacing;

        if next_tick > 887272 {
            panic!("Up Tick OOB")
        } else {
            unsafe { Self::new_unchecked(next_tick) }
        }
    }

    /// Returns the previous initializable tick
    /// 
    /// # Panics: 
    /// - if the next tick is out of bounds
    pub fn down(self, spacing: TickSpacing) -> Self {
        let spacing = spacing as i32;
        let scalar = self.0 / spacing - 1;
        let prev_tick = scalar * spacing;

        if prev_tick < -887272 {
            panic!("Down Tick OOB")
        } else {
            unsafe { Self::new_unchecked(prev_tick) }
        }
    }
}

impl Price {
    /// Creates a new sqrt price from a float
    /// 
    /// # Errors:
    /// - if the price is less than the minimum price
    /// - if the price is greater than the maximum price
    pub fn new(float: Float) -> Result<Self, BoundsError> {
        if float >= *MIN_PRICE && float <= *MAX_PRICE {
            Ok(Self(float))
        } else {
            Err(BoundsError("Price", float.to_string()))
        }
    }

    pub fn invert(self) -> Self {
        Self(self.0.recip())
    }

    /// Creates a new price from a float without checking if it is valid
    /// useful for when youre reading directly from the pool
    pub unsafe fn new_unchecked(float: Float) -> Self {
        Self(float)
    }
}

impl SqrtPrice {
    /// Creates a new sqrt price from a float
    /// 
    /// # Errors:
    /// - if the sqrt price is less than the minimum sqrt price
    /// - if the sqrt price is greater than the maximum sqrt price
    pub fn new(float: Float) -> Result<Self, BoundsError> {
        if float >= *MIN_SQRT_PRICE && float <= *MAX_SQRT_PRICE {
            Ok(Self(float))
        } else {
            Err(BoundsError("SqrtPrice", float.to_string()))
        }
    }

    /// Creates a new sqrt price from a float without checking if it is valid
    /// useful for when youre reading directly from the pool
    pub unsafe fn new_unchecked(float: Float) -> Self {
        Self(float)
    }
}

impl PartialOrd<Float> for Price {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Float> for SqrtPrice {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<Float> for Price {
    fn eq(&self, other: &Float) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Float> for SqrtPrice {
    fn eq(&self, other: &Float) -> bool {
        self.0.eq(other)
    }
}

impl From<Price> for U256 {
    fn from(price: Price) -> Self {
        price.0.into_u256().expect("To convert a valid price wrapper to a u256")
    }
}

impl From<Price> for SqrtPrice {
    fn from(price: Price) -> Self {
        SqrtPrice(price.0.sqrt())
    }
}

impl From<SqrtPrice> for Price {
    fn from(sqrt_price: SqrtPrice) -> Self {
        Price(sqrt_price.0.pow(2))
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for SqrtPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Tick> for Signed<24, 1> {
    fn from(tick: Tick) -> Self {
        Signed::from_str(&tick.0.to_string()).expect("To convert a valid tick to a i24")
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cant_make_oob_tick() {
        let tick = Tick::new(887273);
        assert!(tick.is_err());
    }

    #[test]
    fn test_cant_make_oob_price() {
        let price = Price::new(Float::with_val(100, 1.0001).pow(887273));
        assert!(price.is_err());
    }

    #[test]
    fn test_cant_make_oob_sqrt_price() {
        let sqrt_price = SqrtPrice::new(Float::with_val(100, 1.0001).pow(887273));
        assert!(sqrt_price.is_err());
    }
}