use std::error::Error;
use rug::float::ParseFloatError;

#[derive(Debug, Clone)]
pub enum V3PoolError<E: Error> {
    ParseError(ParseFloatError),
    BackendError(E),
    PoolNotFound,
}

impl<E: Error> V3PoolError<E> {
    pub fn backend_error(e: E) -> Self {
        V3PoolError::BackendError(e)
    }
}

impl<E: Error> From<ParseFloatError> for V3PoolError<E> {
    fn from(e: ParseFloatError) -> Self {
        V3PoolError::ParseError(e)
    }
}

