use crate::pool::{V3Pool, V3PoolError};
use rug::{ops::Pow, Float};

pub enum Token {
    Zero,
    One,
}

/// [Numeraire] is a trait that is implemented on all [V3Pool]s
///
/// It provides some helper functions for getting the price of the pool in "human readable" form
///
/// as well as provides an interface for fetching this in some target quote currency
///
/// todo we should probaly have strong types on the Prices returned
pub trait Numeraire: V3Pool {
    /// returns the human readable price in terms of Token
    fn human_price_in(&self, numeraire: Token) -> Result<Float, V3PoolError<Self::BackendError>> {
        match numeraire {
            Token::Zero => self.inv_human_price(),
            Token::One => self.human_price(),
        }
    }

    /// price of token0 in token terms of token 1, accounting for decimals
    /// aka y / x
    fn human_price(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let price = self.sqrt_price()?.pow(2);

        let exp = -(self.token1_decimals() as i16 - self.token0_decimals() as i16);

        let price = price * Float::with_val(100, 10).pow(exp);

        Ok(price)
    }

    /// price of token0 in token terms of token 1, accounting for decimals
    /// aka y / x
    fn from_human_price(&self, price: Float) -> Result<Float, V3PoolError<Self::BackendError>> {
        let exp = -(self.token1_decimals() as i16 - self.token0_decimals() as i16);

        let price = price / Float::with_val(100, 10).pow(exp);

        Ok(price)
    }

    /// returns the inverse of the huamn readable price from the pool
    fn inv_human_price(&self) -> Result<Float, V3PoolError<Self::BackendError>> {
        let price = self.human_price()?;

        Ok(Float::with_val(100, 1) / price)
    }
}

impl<P: V3Pool> Numeraire for P {}
