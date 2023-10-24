use super::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Agent error: {0}")]
    GenericError(String),
    #[error("RevmMiddleware error: {0}")]
    RevmMiddlewareError(#[from] RevmMiddlewareError),
}

impl From<anyhow::Error> for SimulationError {
    fn from(error: anyhow::Error) -> Self {
        SimulationError::GenericError(error.to_string())
    }
}
