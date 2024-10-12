use crate::math::Tick;
use rug::float::ParseFloatError;
use std::error::Error;
use alloy::primitives::Signed;

use crate::math::BoundsError;
use crate::TickSpacing;

#[derive(Debug, thiserror::Error)]
pub enum V3PoolError<E: Error> {
    #[error("Error parsing float: {0}")]
    ParseError(#[from] ParseFloatError),
    #[error("Backend error: {0}")]
    BackendError(E),
    #[error("Bounds error: {0}")]
    BoundsError(#[from] BoundsError),
    #[error("Pool not found")]
    PoolNotFound,
    #[error("Bad tick range: {0:?} {1:?} {2:?}")]
    BadTickRange(Tick, Tick, Signed<24, 1>),
    #[error("Too many ticks")]
    TooManyTicks,
    #[error("Unsupported Chain")]
    UnsupportedChain,
}

impl<E: Error> V3PoolError<E> {
    pub fn backend_error(e: E) -> Self {
        V3PoolError::BackendError(e)
    }
}
