use rug::ops::Pow;
use rug::Float;

use super::Token;
use crate::math::BoundsError;
use crate::math::Price;
use crate::V3Pool;
use ethers::types::U256;

/// Display and Debug implementations return the human readable price in the form the user selected at creation
/// and there are helpers for converting between human readable prices and scaled prices
///
/// ## Comparions
/// A pool price is directly comparable with other pool prices.
/// If youre working with a price which is not in pool form (say from an oracle), its reccomended to use the
/// [PoolPrice::from_normalized] to convert it to a pool price, or you could use [PoolPrice::normalized] and
/// compare floats directly.
///
/// ## Conversions
/// - all Into<T> implementations will return the scaled price accounting for the internal decimals of the pool
///     - if you want the human readable price use [PoolPrice::normalized]
pub struct PoolPrice<'a, P> {
    /// The token you want the price to be formatted as
    numeraire: Token,

    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Price,

    /// the pool that this price is for
    pool: &'a P,
}

impl<'a, P> Clone for PoolPrice<'a, P> {
    fn clone(&self) -> Self {
        Self {
            price: self.price.clone(),
            pool: self.pool,
            numeraire: self.numeraire,
        }
    }
}

impl<'a, P: V3Pool> PoolPrice<'a, P> {
    /// Create a new pool price from this Price wrapper.
    ///
    /// # Arguments
    /// The price of token0 in terms of token1, and the denomination to format the price in
    pub(crate) fn from_price(pool: &'a P, price: Price, numeraire: Token) -> Self {
        Self {
            price,
            pool,
            numeraire,
        }
    }

    /// Converts a (normalized) human readable price into a pool price
    pub fn from_normalized(
        pool: &'a P,
        price: Float,
        numeraire: Token,
    ) -> Result<Self, BoundsError> {
        match numeraire {
            Token::One => Ok(Self {
                price: Price::new(scale_up(pool, price))?,
                pool,
                numeraire,
            }),
            Token::Zero => Ok(Self {
                price: Price::new(scale_up(pool, price.recip()))?,
                pool,
                numeraire,
            }),
        }
    }

    /// Remove scalar decimals from the price
    ///
    /// In other words this is the human readable price
    pub fn normalized(&self) -> Float {
        match self.numeraire {
            Token::One => self.scale_down(),
            Token::Zero => self.scale_down().recip(),
        }
    }

    /// Trys to create a spread from the pool price erroring if the new prices are out of bounds
    ///
    /// # Returns:
    /// the lower and upper bounds respectively
    #[allow(unused)]
    fn try_create_spread(&self, _bps: u16) -> Result<(Self, Self), BoundsError> {
        todo!()
    }
}

impl<'a, P: V3Pool> PoolPrice<'a, P> {
    /// Probaly only useful for formatting
    ///
    /// Remove the internal decimals from the price
    fn scale_down(&self) -> Float {
        scale_down(self.pool, self.price.clone().into())
    }
}

fn scale_up<P: V3Pool>(pool: &P, val: Float) -> Float {
    let exp = *pool.token1_decimals() as i16 - *pool.token0_decimals() as i16;

    val * Float::with_val(100, 10).pow(exp)
}

fn scale_down<P: V3Pool>(pool: &P, val: Float) -> Float {
    let exp = *pool.token0_decimals() as i16 - *pool.token1_decimals() as i16;

    val * Float::with_val(100, 10).pow(exp)
}

impl<'a, P> From<PoolPrice<'a, P>> for Price {
    fn from(price: PoolPrice<'a, P>) -> Self {
        price.price
    }
}

impl<'a, P> From<PoolPrice<'a, P>> for Float {
    fn from(price: PoolPrice<'a, P>) -> Self {
        price.price.into()
    }
}

impl<'a, P> From<PoolPrice<'a, P>> for U256 {
    fn from(price: PoolPrice<'a, P>) -> Self {
        price.price.into()
    }
}

impl<'a, P: V3Pool> std::fmt::Display for PoolPrice<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.normalized())
    }
}

impl<'a, P: V3Pool> std::fmt::Debug for PoolPrice<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPrice")
            .field("price_normalized", &self.normalized())
            .field("token0", &self.pool.token0())
            .field("token1", &self.pool.token1())
            .field("token0_decimals", &self.pool.token0_decimals())
            .field("token1_decimals", &self.pool.token1_decimals())
            .field("numeraire", &self.numeraire)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use crate::types::tests::MockPool;
    use rug::ops::Pow;

    #[test]
    fn test_price_scaled_as_expected() {
        let pool = MockPool {
            token0: Default::default(),
            token1: Default::default(),
            token0_decimals: 5,
            token1_decimals: 10,
            fee: crate::FeeTier::Mid,
        };

        let price = rug::Float::with_val(100, 100);
        let pool_price = crate::types::price::PoolPrice::from_normalized(
            &pool,
            price.clone(),
            crate::types::Token::One,
        )
        .unwrap();

        println!("{:?}", rug::Float::from(pool_price.clone()).to_string());
        assert_eq!(
            rug::Float::from(pool_price),
            price * rug::Float::with_val(100, 10).pow(5)
        );
    }

    #[test]
    fn test_price_round_trip() {
        let pool = MockPool {
            token0: Default::default(),
            token1: Default::default(),
            token0_decimals: 5,
            token1_decimals: 10,
            fee: crate::FeeTier::Mid,
        };

        let price = rug::Float::with_val(100, 100);
        let pool_price = crate::types::price::PoolPrice::from_normalized(
            &pool,
            price.clone(),
            crate::types::Token::One,
        )
        .unwrap();

        assert_eq!(pool_price.normalized(), price);
    }
}
