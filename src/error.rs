use crate::math::Tick;
use rug::float::ParseFloatError;
use std::error::Error;

use crate::math::BoundsError;
use crate::TickSpacing;

use alloy::contract::MultiCallError;

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
    BadTickRange(Tick, Tick, TickSpacing),
    #[error("Too many ticks")]
    TooManyTicks,
    #[error("Multicall error: {0}")]
    MulticallError(#[from] MultiCallError),
}

impl<E: Error> V3PoolError<E> {
    pub fn backend_error(e: E) -> Self {
        V3PoolError::BackendError(e)
    }
}
