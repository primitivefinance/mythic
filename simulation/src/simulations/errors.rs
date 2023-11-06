use serde_json::Value;
use thiserror::Error;

use super::*;

#[derive(Clone, Error, Debug, Serialize, Deserialize)]
pub enum SimulationError {
    #[error("Generic error: {0}")]
    GenericError(String),
    #[error("Error in simulation: {0}")]
    Error(Value),
}

impl From<anyhow::Error> for SimulationError {
    fn from(error: anyhow::Error) -> Self {
        SimulationError::GenericError(error.to_string())
    }
}
