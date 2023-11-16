use super::{errors::SimulationError, *};
use crate::settings::SimulationConfig;

pub async fn setup(
    environment: Environment,
    config: SimulationConfig<Single>,
) -> Result<Simulation, SimulationError> {
    todo!();
}
