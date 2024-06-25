use rug::ops::Pow;
use rug::Float;

use crate::math::BoundsError;
use crate::math::Price;
use crate::Token;
use crate::V3Pool;

/// A price that belongs to a pool that automatic accounts for the internal decimals
/// it also accounts for the ordering of the tokens in the pool
/// 
/// Display and Debug implementations return the human readable price
/// and there are helpers for converting between human readable prices and scaled prices
/// 
/// By default this 
pub struct PoolPrice<'a, P: V3Pool> {
    numeraire: Token,

    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Price,

    /// the pool that this price is for
    pool: &'a P,
}

impl<'a, P: V3Pool> PoolPrice<'a, P> {
    pub fn from_price_wrapper(pool: &'a P, price: Price, numeraire: Token) -> Self {
        match numeraire {
            Token::One => Self {
                price,
                pool,
                numeraire,
            },
            Token::Zero => Self {
                // saftey: it came from a valid price to start with
                price: unsafe { Price::new_unchecked(price.into_inner().recip()) },
                pool,
                numeraire,
            },
        }
    }

    /// Converts a human readable price into a pool price
    pub fn from_human_readable(
        pool: &'a P,
        price: Float,
        numeraire: Token,
    ) -> Result<Self, BoundsError> {
        match numeraire {
            Token::One => Ok(Self {
                price: Price::new(Self::scale_up(pool, price))?,
                pool,
                numeraire,
            }),
            Token::Zero => Ok(Self {
                price: Price::new(Self::scale_up(pool, price.recip()))?,
                pool,
                numeraire,
            }),
        }
    }

    /// Returns the human readable price in terms of some token
    pub fn price_in(&self, numeraire: Token) -> Float {
        match numeraire {
            Token::One => self.scale_down(),
            Token::Zero => self.scale_down().recip(),
        }
    }

    /// Consumes the pool price and returns the validated price for this pool
    pub fn into_price(self) -> Price {
        self.price
    }
}

impl<'a, P: V3Pool> PoolPrice<'a, P> {
    /// Account for the internal decimals of the pool
    fn scale_up(pool: &P, val: Float) -> Float {
        let exp = *pool.token1_decimals() as i16 - *pool.token0_decimals() as i16;

        val * Float::with_val(100, 10).pow(exp)
    }

    /// Probaly only useful for formatting
    ///
    /// Remove the internal decimals from the price
    fn scale_down(&self) -> Float {
        let exp = *self.pool.token0_decimals() as i16 - *self.pool.token1_decimals() as i16;

        self.price.clone() * Float::with_val(100, 10).pow(exp)
    }
}

impl<'a, P: V3Pool> std::ops::Deref for PoolPrice<'a, P> {
    type Target = Price;

    fn deref(&self) -> &Self::Target {
        &self.price
    }
}

impl<'a, P: V3Pool> From<PoolPrice<'a, P>> for Price {
    fn from(price: PoolPrice<'a, P>) -> Self {
        price.price
    }
}

impl<'a, P: V3Pool> From<PoolPrice<'a, P>> for Float {
    fn from(price: PoolPrice<'a, P>) -> Self {
        price.price.into()
    }
}

impl<'a, P: V3Pool> std::fmt::Display for PoolPrice<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.price_in(self.numeraire))
    }
}

impl<'a, P: V3Pool> std::fmt::Debug for PoolPrice<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPrice")
            .field("price", &self.price)
            .field("token0", &self.pool.token0())
            .field("token1", &self.pool.token1())
            .field("token0_decimals", &self.pool.token0_decimals())
            .field("token1_decimals", &self.pool.token1_decimals())
            .finish()
    }
}
