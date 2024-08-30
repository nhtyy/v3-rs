use std::error::Error;
use ethers::contract::MulticallError;
use rug::float::ParseFloatError;
use crate::math::Tick;

use crate::math::BoundsError;
use crate::TickSpacing;

#[derive(Debug, Clone)]
pub enum V3PoolError<E: Error> {
    ParseError(ParseFloatError),
    BackendError(E),
    BoundsError(BoundsError),
    PoolNotFound,
    BadTickRange(Tick, Tick, TickSpacing),
    TooManyTicks,
    MulticallError(String),
}

impl<E: Error> Error for V3PoolError<E> {}

impl<E: Error> V3PoolError<E> {
    pub fn backend_error(e: E) -> Self {
        V3PoolError::BackendError(e)
    }
}

impl<E: Error> std::fmt::Display for V3PoolError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            V3PoolError::ParseError(e) => write!(f, "V3PoolError::ParseError: {}", e),
            V3PoolError::BackendError(e) => write!(f, "V3PoolError::BackendError: {}", e),
            V3PoolError::BoundsError(e) => write!(f, "V3PoolError::BoundsError: {}", e),
            V3PoolError::PoolNotFound => write!(f, "V3PoolError: No Pool found"),
            V3PoolError::BadTickRange(a, b, c) => write!(f, "V3PoolError: Bad tick range: {:?} {:?} {:?}", a, b, c),
            V3PoolError::TooManyTicks => write!(f, "V3PoolError: Too many ticks"),
            V3PoolError::MulticallError(e) => write!(f, "V3PoolError: Multicall error: {}", e),
        }
    }
}

impl<E: Error> From<BoundsError> for V3PoolError<E> {
    fn from(e: BoundsError) -> Self {
        V3PoolError::BoundsError(e)
    }
}

impl<E: Error> From<ParseFloatError> for V3PoolError<E> {
    fn from(e: ParseFloatError) -> Self {
        V3PoolError::ParseError(e)
    }
}

