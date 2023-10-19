use crate::pool::{V3Pool, V3PoolError};
use rug::{ops::Pow, Float};

pub enum Token {
    Zero,
    One,
}

pub struct PoolPrice<'pool, P: V3Pool> {
    /// the price of token0 in terms of token1, accounting for the decimals of the pool
    price: Float,

    /// the pool that this price is for
    pool: &'pool P,
}

impl<'pool, P: V3Pool> PoolPrice<'pool, P> {
    /// creates a new [PoolPrice] from a pool and a price
    /// the price should not attempt to account for decimals
    pub fn new(pool: &'pool P, price: Float, numeraire: Token) -> Self {
        match numeraire {
            Token::One => Self { price, pool },
            Token::Zero => Self {
                price: Float::with_val(100, 1) / price,
                pool,
            },
        }
    }

    /// consumes the type and returns the pool price accounting for the decimals of the pool
    pub fn into_pool_price_float(self) -> Float {
        let exp = self.pool.token1_decimals() as i16 - self.pool.token0_decimals() as i16;

        self.price * Float::with_val(100, 10).pow(exp)
    }

    /// returns the human readable price in terms of some numeriare
    pub fn price_in(&self, numeraire: Token) -> Float {
        match numeraire {
            Token::One => self.price.clone(),
            Token::Zero => Float::with_val(100, 1) / self.price.clone(),
        }
    }
}

/// [Numeraire] is a trait that is implemented on all [V3Pool]s
///
/// It provides some helper functions for getting the price of the pool in "human readable" form
///
/// as well as provides an interface for fetching this in some target quote currency
#[async_trait::async_trait]
pub trait Numeraire: V3Pool + Sized {
    async fn pool_price(&self) -> Result<PoolPrice<Self>, V3PoolError<Self::BackendError>> {
        let price = self.sqrt_price().await?.pow(2);

        let exp = -(self.token1_decimals() as i16 - self.token0_decimals() as i16);

        let price = price * Float::with_val(100, 10).pow(exp);

        Ok(PoolPrice::new(self, price, Token::One))
    }
}

impl<P: V3Pool> Numeraire for P {}
