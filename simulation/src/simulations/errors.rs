use thiserror::Error;

use super::*;

#[derive(Clone, Error, Debug, Serialize, Deserialize)]
pub enum SimulationError {
    #[error("Agent error: {0}")]
    GenericError(String),
}

impl From<anyhow::Error> for SimulationError {
    fn from(error: anyhow::Error) -> Self {
        SimulationError::GenericError(error.to_string())
    }
}
