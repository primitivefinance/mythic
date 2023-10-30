use serde_json::error;
use tokio::runtime::Runtime;

use self::errors::SimulationError;
use super::*;
use crate::{
    agents::{Agent, Agents},
    settings::parameters::Fixed,
};

pub mod dynamic_weights;
pub mod errors;
pub mod momentum;
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
    MomentumStrategy,
}

impl SimulationType {
    async fn run(config: SimulationConfig<Fixed>) -> Result<(), SimulationError> {
        let simulation = match config.simulation {
            SimulationType::DynamicWeights => dynamic_weights::setup(config.clone()).await?,
            SimulationType::StablePortfolio => stable_portfolio::setup(config.clone()).await?,
            SimulationType::MomentumStrategy => momentum::setup(config.clone()).await?,
        };
        match looper(simulation.agents, simulation.steps).await {
            Ok(_) => {
                simulation.environment.stop();
                Ok(())
            }
            Err(e) => {
                let metadata = format!(
                    "{}_{}",
                    config.output_directory,
                    config.output_file_name.unwrap()
                );
                let error_string = format!("Error in simulation `{:?}`: {:?}", metadata, e);
                error!(error_string);
                simulation.environment.stop();
                Err(SimulationError::GenericError(error_string))
            }
        }
    }
}

use tokio::sync::Semaphore;

pub fn batch(config_path: &str) -> Result<()> {
    let config = SimulationConfig::new(config_path)?;

    let direct_configs: Vec<SimulationConfig<Fixed>> = config.generate();
    warn!("Running {} simulations", direct_configs.len());

    // Create a multi-threaded runtime
    let rt = Builder::new_multi_thread().build()?;

    // Create a semaphore with a given number of permits
    let semaphore = config
        .max_parallel
        .map(|max_parallel| Arc::new(Semaphore::new(max_parallel)));

    rt.block_on(async {
        let mut handles = vec![];
        let errors = Arc::new(tokio::sync::Mutex::new(vec![]));

        for config in direct_configs {
            let errors_clone = errors.clone();
            let semaphore_clone = semaphore.clone();
            handles.push(tokio::spawn(async move {
                // Acquire a permit inside the spawned task
                let permit = if let Some(ref semaphore_clone) = semaphore_clone {
                    // Acquire a permit outside the spawned task
                    let permit = semaphore_clone.acquire().await.unwrap();
                    Some(permit)
                } else {
                    None
                };

                warn!("Running simulation with config: {:?}", config);
                let result = SimulationType::run(config).await;
                match result {
                    Err(e) => {
                        let mut errors_clone_lock = errors_clone.lock().await;
                        errors_clone_lock.push(e);
                        // Drop the permit when the simulation is done.
                        drop(permit);
                    }
                    Ok(_) => {
                        drop(permit);
                    }
                }
            }));
        }

        for handle in handles {
            handle.await?;
            warn!("Simulation complete");
        }

        let error_path = config.output_directory.clone() + "/errors.json";
        serde_json::to_writer(
            std::fs::File::create(error_path).unwrap(),
            &*errors.lock().await,
        );

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
            .join("test_static/gbm_drift=0.1_vol=0.35/trajectory=0.json");
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
                        "test_sweep/gbm_drift={}_vol={}/trajectory={}.json",
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

        // std::fs::remove_dir_all("test_sweep").unwrap();
    }
}
