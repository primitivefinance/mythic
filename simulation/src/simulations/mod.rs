use tokio::runtime::Runtime;
use tracing::warn;

use self::errors::SimulationError;

use super::*;

use crate::settings::parameters::Direct;

pub mod dynamic_weights;
pub mod errors;
pub mod stable_portfolio;
use settings::parameters::Parameterized;
use tokio::runtime::Builder;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationType {
    DynamicWeights,
    StablePortfolio,
}

impl SimulationType {
    async fn run(config: SimulationConfig<Direct>) -> Result<(), SimulationError> {
        match config.simulation {
            SimulationType::DynamicWeights => dynamic_weights::run(config).await,
            SimulationType::StablePortfolio => stable_portfolio::run(config).await,
        }
    }
}

pub fn batch(config_path: &str) -> Result<()> {
    let config = SimulationConfig::new(config_path)?;

    let direct_configs: Vec<SimulationConfig<Direct>> = config.generate();
    warn!("Running {} simulations", direct_configs.len());
    let mut rt = Builder::new_multi_thread().build()?;
    let mut handles = vec![];

    for config in direct_configs {
        warn!("Running simulation with config: {:?}", config);
        handles.push(rt.spawn(SimulationType::run(config)));
    }

    rt.block_on(async {
        for handle in handles {
            handle.await?;
            warn!("Simulation complete");
        }
        Ok(())
    })
}
