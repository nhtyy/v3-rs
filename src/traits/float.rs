use alloy::primitives::Uint;
use alloy::primitives::U256;
use rug::ops::Pow;
use rug::Float;

lazy_static::lazy_static! {
    pub static ref U256_MAX: Float = Float::with_val(100, 2).pow(256);
}

pub trait IntoFloat {
    fn into_float(&self) -> Float;
}

impl<const B: usize, const L: usize> IntoFloat for Uint<B, L> {
    fn into_float(&self) -> Float {
        // safey: U256 is always a valid float
        let parse = Float::parse(self.to_string()).expect("Failed to parse U256 to Float");

        Float::with_val(100, parse)
    }
}

impl IntoFloat for f64 {
    fn into_float(&self) -> Float {
        Float::with_val(100, *self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Value is negative")]
    ValueNegative,
    #[error("Value is too large")]
    ValueTooLarge,
    #[error("Value is NaN")]
    NaN,
    #[error("Value is infinite")]
    IsInfinte,
}

#[doc(hidden)]
/// We want to impl TryFrom<Float> for U256 but orphan rule
///
/// Makes the bounds nicer anyway
pub trait IntoU256 {
    /// Converts a Float to a U256
    ///
    /// # Errors:
    /// - If the float is not a valid U256
    fn into_u256(&self) -> Result<U256, ConversionError>;
}

impl IntoU256 for Float {
    #[inline]
    fn into_u256(&self) -> Result<U256, ConversionError> {
        if self.is_infinite() {
            return Err(ConversionError::IsInfinte);
        }

        if self.is_nan() {
            return Err(ConversionError::NaN);
        }

        if self.is_sign_negative() {
            return Err(ConversionError::ValueNegative);
        }

        if self > &U256_MAX.clone() {
            return Err(ConversionError::ValueTooLarge);
        }

        // rounds floor
        let i = self
            .to_integer()
            .expect("Failed to convert Float to integer even though done our checks, this is a bug")
            .to_string();

        Ok(U256::from_str_radix(&i, 10).expect(
            "Failed to convert Float to U256 even though weve done our checks, this is a bug",
        ))
    }
}
