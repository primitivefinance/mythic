use tokio::runtime::Runtime;

use self::errors::SimulationError;

use super::*;

use crate::settings::parameters::Direct;

pub mod dynamic_weights;
pub mod errors;
pub mod stable_portfolio;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SimulationType {
    DynamicWeights,
    StablePortfolio,
}

impl SimulationType {
    async fn run(config: SimulationConfig<Direct>) -> Result<(), SimulationError> {
        match config.simulation_type {
            SimulationType::DynamicWeights => dynamic_weights::run(config).await,
            SimulationType::StablePortfolio => stable_portfolio::run(config).await,
        }
    }
}

pub fn batch(config_path: &str) -> Result<()> {
    let config = SimulationConfig::new(config_path)?;

    let direct_configs = config.to_direct_configs();
    let mut rt = Runtime::new()?;
    let mut handles = vec![];

    for config in direct_configs {
        handles.push(rt.spawn(SimulationType::run(config)));
    }

    rt.block_on(async {
        for handle in handles {
            handle.await?;
        }
        Ok(())
    })
}
