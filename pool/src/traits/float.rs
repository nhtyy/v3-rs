use ethers::types::U256;
use rug::Float;
use rug::ops::Pow;

lazy_static::lazy_static! {
    pub static ref U256_MAX: Float = Float::with_val(100, 2).pow(256);
}

pub(crate) trait IntoFloat {
    fn into_float(&self) -> Float;
}

impl IntoFloat for U256 {
    fn into_float(&self) -> Float {
        // safey: U256 is always a valid float
        let parse = Float::parse(self.to_string()).expect("Failed to parse U256 to Float");

        Float::with_val(100, parse)
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

pub trait IntoU256 {
    /// Converts a Float to a U256
    ///
    /// # Errors:
    /// - If the float is not a valid U256
    fn into_u256(&self) -> Result<U256, ConversionError>;
}

impl IntoU256 for Float {
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
        let i = self.to_integer().expect("Not converting inf to U256");

        Ok(U256::from_dec_str(&i.to_string()).unwrap())
    }
}
