use rug::ops::Pow;
use rug::Float;

use super::Token;
use crate::math::BoundsError;
use crate::math::Price;
use crate::V3Pool;
use ethers::types::U256;

pub mod numeraire {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Zero;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct One;
}
pub use numeraire::*;

impl sealed::Sealed for Zero {}
impl sealed::Sealed for One {}

impl Numeraire for Zero {
    fn token() -> Token {
        Token::Zero
    }
}

impl Numeraire for One {
    fn token() -> Token {
        Token::One
    }
}

/// A marker trait for the types [Zero] and [One],
/// which indicate the token that a [PoolPrice] is denominated in
pub trait Numeraire: sealed::Sealed {
    fn token() -> Token;
}

mod sealed {
    pub trait Sealed {}
}

/// Display and Debug implementations return the human readable price in the form the user selected at creation
/// and there are helpers for converting between human readable prices and scaled prices
///
/// A pool price is generic over a type N: Numeraire which is a marker trait for the token that the price is denominated in
/// A pool price implements from
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
pub struct PoolPrice<'a, P, N> {
    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Price,
    /// the pool that this price is for
    pool: &'a P,
    /// the token that the price is denominated in
    _numeraire: std::marker::PhantomData<N>,
}

impl<'a, P, N> Clone for PoolPrice<'a, P, N> {
    fn clone(&self) -> Self {
        Self {
            price: self.price.clone(),
            pool: self.pool,
            _numeraire: self._numeraire,
        }
    }
}

impl<'a, P: V3Pool, N: Numeraire> PoolPrice<'a, P, N> {
    /// Create a new pool price from this Price wrapper.
    ///
    /// # Arguments
    /// The price of token0 in terms of token1, and the denomination to format the price in
    pub fn from_price(pool: &'a P, price: Price) -> Self {
        Self {
            price,
            pool,
            _numeraire: std::marker::PhantomData,
        }
    }

    /// Converts a (normalized) human readable price into a pool price
    pub fn from_normalized(pool: &'a P, price: Float) -> Result<Self, BoundsError> {
        match N::token() {
            Token::One => Ok(Self {
                price: Price::new(scale_up(pool, price))?,
                pool,
                _numeraire: std::marker::PhantomData,
            }),
            Token::Zero => Ok(Self {
                price: Price::new(scale_up(pool, price.recip()))?,
                pool,
                _numeraire: std::marker::PhantomData,
            }),
        }
    }

    /// Remove scalar decimals from the price
    ///
    /// In other words this is the human readable price
    pub fn normalized(&self) -> Float {
        match N::token() {
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

impl<'a, P: V3Pool, N> PoolPrice<'a, P, N> {
    /// Probaly only useful for formatting
    ///
    /// Remove the internal decimals from the price
    fn scale_down(&self) -> Float {
        scale_down(self.pool, self.price.clone().into())
    }
}

impl<'a, P, N> From<PoolPrice<'a, P, N>> for Price {
    fn from(price: PoolPrice<'a, P, N>) -> Self {
        price.price
    }
}

impl<'a, P, N> From<PoolPrice<'a, P, N>> for Float {
    fn from(price: PoolPrice<'a, P, N>) -> Self {
        price.price.into()
    }
}

impl<'a, P, N> From<PoolPrice<'a, P, N>> for U256 {
    fn from(price: PoolPrice<'a, P, N>) -> Self {
        price.price.into()
    }
}

/// Scales a value to account for the decimals of the pool assuming its in the correct numeraire for the pool ordering
fn scale_up<P: V3Pool>(pool: &P, val: Float) -> Float {
    let exp = *pool.token1_decimals() as i16 - *pool.token0_decimals() as i16;

    val * Float::with_val(100, 10).pow(exp)
}

/// Scales a value to account for the decimals of the pool assuming its in the correct numeraire for the pool ordering
fn scale_down<P: V3Pool>(pool: &P, val: Float) -> Float {
    let exp = *pool.token0_decimals() as i16 - *pool.token1_decimals() as i16;

    val * Float::with_val(100, 10).pow(exp)
}

//////////////////////// Comparsion ////////////////////////

impl<'a, P: V3Pool, N> PartialEq for PoolPrice<'a, P, N> {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(debug_assertions)]
        {
            if self.pool.address() != other.pool.address() {
                tracing::warn!("comparing pool prices for different pools");
                tracing::warn!("pool1: {:?}", self.pool.address());
                tracing::warn!("pool2: {:?}", other.pool.address());
            }
        }

        self.price.eq(&other.price)
    }
}

impl<'a, P: V3Pool, N> PartialOrd for PoolPrice<'a, P, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        #[cfg(debug_assertions)]
        {
            if self.pool.address() != other.pool.address() {
                tracing::warn!("comparing pool prices for different pools");
                tracing::warn!("pool1: {:?}", self.pool.address());
                tracing::warn!("pool2: {:?}", other.pool.address());
            }
        }

        self.price.partial_cmp(&other.price)
    }
}

impl<'a, P: V3Pool, N: Numeraire> std::fmt::Display for PoolPrice<'a, P, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.normalized())
    }
}

impl<'a, P: V3Pool, N: Numeraire> std::fmt::Debug for PoolPrice<'a, P, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPrice")
            .field("price", &self.normalized())
            .field("token0", &self.pool.token0())
            .field("token1", &self.pool.token1())
            .field("token0_decimals", &self.pool.token0_decimals())
            .field("token1_decimals", &self.pool.token1_decimals())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::One;
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
        let pool_price =
            crate::types::price::PoolPrice::<_, One>::from_normalized(&pool, price.clone())
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
        let pool_price =
            crate::types::price::PoolPrice::<_, One>::from_normalized(&pool, price.clone())
                .unwrap();

        assert_eq!(pool_price.normalized(), price);
    }
}
