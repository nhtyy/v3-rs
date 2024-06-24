use lazy_static::lazy_static;
use rug::Float;
use rug::ops::Pow;

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

/// Implements conversions and operations on the type
/// 
/// The type must impl `Add`, `Sub`, `Mul`, `Div`
macro_rules! impl_wrappers {
    (
        $(
            $(#[$attrs:meta])*
            pub struct $name:ident($vis:vis $inner:ty);
        )*
    ) => {
        $(
            $(#[$attrs])*
            pub struct $name($vis $inner);

            impl $name {
                #[inline]
                /// Get the wrapped value by consuming the wrapper
                pub fn into_inner(self) -> $inner {
                    self.0
                }
            }

            impl From<$name> for $inner {
                fn from(inner: $name) -> Self {
                    inner.0
                }
            }

            impl ::std::ops::Deref for $name {
                type Target = $inner;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl ::std::ops::Add for $name {
                type Output = $inner;

                fn add(self, rhs: Self) -> Self::Output {
                    self.0 + rhs.0
                }
            }

            impl ::std::ops::Add<$inner> for $name {
                type Output = $inner;

                fn add(self, rhs: $inner) -> Self::Output {
                    self.0 + rhs
                }
            }

            impl ::std::ops::Add<$name> for $inner {
                type Output = $inner;

                fn add(self, rhs: $name) -> Self::Output {
                    self + rhs.0
                }
            }

            impl ::std::ops::Sub for $name {
                type Output = $inner;

                fn sub(self, rhs: Self) -> Self::Output {
                    self.0 - rhs.0
                }
            }

            impl ::std::ops::Sub<$inner> for $name {
                type Output = $inner;

                fn sub(self, rhs: $inner) -> Self::Output {
                    self.0 - rhs
                }
            }

            impl ::std::ops::Sub<$name> for $inner {
                type Output = $inner;

                fn sub(self, rhs: $name) -> Self::Output {
                    self - rhs.0
                }
            }

            impl ::std::ops::Mul for $name {
                type Output = $inner;

                fn mul(self, rhs: Self) -> Self::Output {
                    self.0 * rhs.0
                }
            }

            impl ::std::ops::Mul<$inner> for $name {
                type Output = $inner;

                fn mul(self, rhs: $inner) -> Self::Output {
                   self.0 * rhs
                }
            }

            impl ::std::ops::Mul<$name> for $inner {
                type Output = $inner;

                fn mul(self, rhs: $name) -> Self::Output {
                    self * rhs.0
                }
            }

            impl ::std::ops::Div for $name {
                type Output = $inner;

                fn div(self, rhs: Self) -> Self::Output {
                    self.0 / rhs.0
                }
            }

            impl ::std::ops::Div<$inner> for $name {
                type Output = $inner;

                fn div(self, rhs: $inner) -> Self::Output {
                   self.0 / rhs
                }
            }

            impl ::std::ops::Div<$name> for $inner {
                type Output = $inner;

                fn div(self, rhs: $name) -> Self::Output {
                    self / rhs.0
                }
            }
        )*
    };
}

impl_wrappers! {
    /// A *valid* tick in the range [-887272, 887272]
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
    pub struct Tick(i32);

    /// A raw validatated pool price
    /// 
    /// This type is meant to be used for raw pool prices
    /// in most cases the account has additional scalars
    /// that can affect the value of this number
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Price(Float);

    /// A raw pool sqrt price
    /// 
    /// This type is meant to be used for raw pool sqrt prices
    /// in most cases the account has additional scalars
    /// that can affect the value of this number
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct SqrtPrice(Float);
}

impl Tick {
    /// Creates a new *valid* tick from an i32
    /// 
    /// # Returns None:
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

    /// Returns the next tick
    /// 
    /// Warning: this function will not return a tick greater than 887272
    pub fn up(self, spacing: TickSpacing) -> Self {
        let next = self.0 + spacing as i32;

        if next > 887272 {
            Self(887272)
        } else {
            self
        }
    }

    /// Returns the previous tick
    /// 
    /// Warning: this function will not return a tick less than -887272
    pub fn down(self, spacing: TickSpacing) -> Self {
        let prev = self.0 - spacing as i32;

        if prev < -887272 {
            Self(-887272)
        } else {
            self
        }
    }
}

impl Price {
    /// Creates a new price from a float
    pub fn new(float: Float) -> Result<Self, BoundsError> {
        if float >= *MIN_PRICE && float <= *MAX_PRICE {
            Ok(Self(float))
        } else {
            Err(BoundsError("Price", float.to_string()))
        }
    }

    /// Creates a new price from a float without checking if it is valid
    /// useful for when youre reading directly from the pool
    pub unsafe fn new_unchecked(float: Float) -> Self {
        Self(float)
    }
}

impl SqrtPrice {
    /// Creates a new sqrt price from a float
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

impl From<Float> for Price {
    fn from(float: Float) -> Self {
        Self(float)
    }
}

impl From<Float> for SqrtPrice {
    fn from(float: Float) -> Self {
        Self(float)
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