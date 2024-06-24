use std::path::Display;

use rug::ops::Pow;
use rug::Float;

use crate::math::BoundsError;
use crate::math::Price;
use crate::Token;
use crate::V3Pool;

pub struct PoolPrice<'pool, P: V3Pool> {
    numeraire: Token,

    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Price,

    /// the pool that this price is for
    pool: &'pool P,
}

impl<'pool, P: V3Pool> PoolPrice<'pool, P> {
    pub fn from_price_wrapper(pool: &'pool P, price: Price, numeraire: Token) -> Self {
        match numeraire {
            Token::One => Self { price, pool, numeraire },
            Token::Zero => Self {
                // saftey: it came from a valid price to start with
                price: unsafe { Price::new_unchecked(price.into_inner().recip()) },
                pool,
                numeraire,
            },
        }
    }

    /// Converts a human readable price into a pool price
    pub fn from_human_readable(pool: &'pool P, price: Float, numeraire: Token) -> Result<Self, BoundsError> {
        match numeraire {
            Token::One => Ok(Self {
                price: Price::new(Self::scale_up(pool, price))?,
                pool,
                numeraire
            }),
            Token::Zero => Ok(Self {
                price: Price::new(Self::scale_up(pool, price.recip()))?,
                pool,
                numeraire
            }),
        }
    }

    fn scale_up(pool: &P, val: Float) -> Float {
        let exp = pool.token1_decimals() as i16 - pool.token0_decimals() as i16;

        val * Float::with_val(100, 10).pow(exp)
    }

    /// Probaly only useful for formatting
    fn scale_down(&self) -> Float {
        let exp = self.pool.token0_decimals() as i16 - self.pool.token1_decimals() as i16;

        self.price.clone() * Float::with_val(100, 10).pow(exp)
    }

    /// returns the human readable price in terms of some numeriare
    pub fn price_in(&self, numeraire: Token) -> Float {
        match numeraire {
            Token::One => self.scale_down(),
            Token::Zero => self.scale_down().recip(),
        }
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
