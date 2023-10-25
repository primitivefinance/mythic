use tokio::runtime::Runtime;
use tracing::{debug, warn};

use self::errors::SimulationError;
use super::*;
use crate::{
    agents::{Agent, Agents},
    settings::parameters::Fixed,
};

pub mod dynamic_weights;
pub mod errors;
pub mod stable_portfolio;
use settings::parameters::Parameterized;
use tokio::runtime::Builder;

pub struct Simulation {
    pub agents: Agents,
    pub steps: usize,
    environment: Environment,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationType {
    DynamicWeights,
    StablePortfolio,
}

impl SimulationType {
    async fn run(config: SimulationConfig<Fixed>) -> Result<(), SimulationError> {
        let simulation = match config.simulation {
            SimulationType::DynamicWeights => dynamic_weights::setup(config).await?,
            SimulationType::StablePortfolio => stable_portfolio::setup(config).await?,
        };
        looper(simulation.agents, simulation.steps).await?;
        Ok(())
    }
}

pub fn batch(config_path: &str) -> Result<()> {
    let config = SimulationConfig::new(config_path)?;

    let direct_configs: Vec<SimulationConfig<Fixed>> = config.generate();
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

pub async fn looper(mut agents: Agents, steps: usize) -> Result<()> {
    info!("Entering startup loop for agents.");

    for agent in agents.iter_mut() {
        agent.startup().await?;
    }

    for index in 0..steps {
        debug!("Entering priority loop for index: {}", index);
        for agent in agents.iter_mut() {
            agent.priority_step().await?;
        }

        debug!("Entering core loop for index: {}", index);
        for agent in agents.iter_mut() {
            agent.step().await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, io::Read, path::Path};

    use super::*;

    #[test]
    fn static_output() {
        batch("configs/test/static.toml").unwrap();
        let path = Path::new(env::current_dir().unwrap().to_str().unwrap())
            .join("test_static/gbm_drift=0.1_vol=0.35/trajectory=0")
            .join("g3m")
            .join("SwapFilter.csv");
        println!("path: {:?}", path);
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        assert!(!contents.is_empty());
        std::fs::remove_dir_all("test_static").unwrap();
    }

    #[test]
    fn sweep_output() {
        batch("configs/test/sweep.toml").unwrap();

        for drift in [-1, 1] {
            for vol in [0, 1] {
                for trajectory in [0, 1] {
                    let str = format!(
                        "test_sweep/gbm_drift={}_vol={}/trajectory={}/g3m/SwapFilter.csv",
                        drift, vol, trajectory
                    );
                    let path = Path::new(env::current_dir().unwrap().to_str().unwrap()).join(str);
                    println!("path: {:?}", path);
                    let mut file = std::fs::File::open(path).unwrap();
                    let mut contents = vec![];
                    file.read_to_end(&mut contents).unwrap();
                    assert!(!contents.is_empty());
                }
            }
        }

        std::fs::remove_dir_all("test_sweep").unwrap();
    }
}
