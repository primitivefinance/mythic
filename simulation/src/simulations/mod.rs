use std::{collections::HashMap, env, path::Path};

use arbiter_core::environment::builder::BlockSettings;
use serde_json::error;
use tokio::{runtime::Runtime, sync::Semaphore};

use self::errors::SimulationError;
use super::*;
use crate::{
    agents::{Agent, Agents},
    settings::parameters::{Multiple, Single},
    strategy::rmm::RmmStrategy,
};

pub mod dynamic_weights;
pub mod errors;
pub mod rmm_vol_targeting;
pub mod stable_portfolio;
use settings::parameters::Parameterized;
use tokio::runtime::Builder;

pub struct World {
    pub simulations: HashMap<String, Simulation>,
}

pub struct WorldBuilder {
    simulations: Option<HashMap<String, Simulation>>,
}

impl Default for SimulationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WorldBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder { simulations: None }
    }

    pub fn simulations(&mut self, simulations: HashMap<String, Simulation>) -> &mut WorldBuilder {
        self.simulations = Some(simulations);
        self
    }

    pub fn build(&mut self) -> Result<World, &'static str> {
        if self.simulations.is_none() {
            return Err("Simulations not set");
        }
        Ok(World {
            simulations: self.simulations.take().unwrap(),
        })
    }
}

pub struct Simulation {
    pub agents: Agents,
    pub steps: usize,
    environment: Environment,
}
pub struct SimulationBuilder {
    agents: Option<Agents>,
    steps: Option<usize>,
    environment: Option<Environment>,
}

impl SimulationBuilder {
    pub fn new() -> SimulationBuilder {
        SimulationBuilder {
            agents: None,
            steps: None,
            environment: None,
        }
    }

    pub fn agents(&mut self, agents: Agents) -> &mut SimulationBuilder {
        self.agents = Some(agents);
        self
    }

    pub fn steps(&mut self, steps: usize) -> &mut SimulationBuilder {
        self.steps = Some(steps);
        self
    }

    pub fn environment(&mut self, environment: Environment) -> &mut SimulationBuilder {
        self.environment = Some(environment);
        self
    }

    pub fn build(&mut self) -> Result<Simulation, &'static str> {
        if self.agents.is_none() {
            return Err("Agents not set");
        }
        if self.steps.is_none() {
            return Err("Steps not set");
        }
        if self.environment.is_none() {
            return Err("Environment not set");
        }
        Ok(Simulation {
            agents: self.agents.take().unwrap(),
            steps: self.steps.take().unwrap(),
            environment: self.environment.take().unwrap(),
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SimulationType {
    #[default]
    DynamicWeights,
    StablePortfolio,
    RmmVolatilityTargeting,
}

impl SimulationType {
    async fn run(config: SimulationConfig<Single>) -> Result<(), SimulationError> {
        let environment = EnvironmentBuilder::new()
            .block_settings(BlockSettings::UserControlled)
            .label(config.output_file_name.clone().unwrap())
            .build();
        let simulation = match config.simulation {
            SimulationType::DynamicWeights => {
                dynamic_weights::setup(environment, config.clone()).await?
            }
            SimulationType::StablePortfolio => {
                stable_portfolio::setup(environment, config.clone()).await?
            }
            SimulationType::RmmVolatilityTargeting => {
                rmm_vol_targeting::setup(environment, config.clone()).await?
            }
        };
        match looper(simulation.agents, simulation.steps).await {
            Ok(_) => {
                simulation.environment.stop();
                Ok(())
            }
            Err(e) => {
                let metadata = serde_json::to_value(&config)
                    .map_err(|e| SimulationError::GenericError(e.to_string()))?;
                error!(
                    { info = e.to_string() },
                    "Error in simulation {:?}",
                    serde_json::to_string(&metadata).unwrap()
                );
                simulation.environment.stop();
                Err(SimulationError::Error(metadata))
            }
        }
    }
}

pub fn from_config(config_path: &str) -> Result<SimulationConfig<Multiple>, ConfigError> {
    let cwd = env::current_dir().unwrap();
    let path = Path::new(cwd.to_str().unwrap());
    let path = path.join(config_path);
    println!("Reading config from: {:?}", path);
    SimulationConfig::new(config_path)
}

pub fn batch(config: SimulationConfig<Multiple>) -> Result<()> {
    let direct_configs: Vec<SimulationConfig<Single>> = config.clone().into();
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

                warn!(
                    "Running environment with label: {}",
                    config.output_file_name.clone().unwrap(),
                );
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

        let output_dir_path =
            std::path::Path::new(std::env::current_dir().unwrap().to_str().unwrap())
                .join(&config.output_directory)
                .to_str()
                .unwrap()
                .to_string();
        std::fs::create_dir_all(output_dir_path)?;
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
        agent.1.startup().await?;
    }

    info!("Entering main loop for agents for {} steps.", steps);
    for index in 0..steps {
        debug!("Entering priority loop for index: {}", index);
        for agent in agents.iter_mut() {
            agent.1.priority_step().await?;
        }

        debug!("Entering core loop for index: {}", index);
        for agent in agents.iter_mut() {
            agent.1.step().await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        agents::{
            block_admin::{BlockAdmin, BlockAdminParametersBuilder},
            price_changer::PriceChanger,
            token_admin::TokenAdmin,
        },
        settings::SimulationConfigBuilder,
    };

    const BLOCK_ADMIN_LABEL: &str = "block_admin";
    const TOKEN_ADMIN_LABEL: &str = "token_admin";
    const PRICE_CHANGER_LABEL: &str = "price_changer";

    #[tokio::test]
    async fn test_simulation_builder() {
        let mut builder = SimulationBuilder::new();
        let environment = EnvironmentBuilder::new()
            .block_settings(BlockSettings::UserControlled)
            .build(); // Replace with your actual initialization

        let block_admin_params = BlockAdminParametersBuilder::new()
            .timestep_size(1)
            .build()
            .unwrap();
        let block_admin = BlockAdmin::new(&environment, block_admin_params, BLOCK_ADMIN_LABEL)
            .await
            .unwrap();
        // let token_admin = TokenAdmin::new(&environment, &config,
        // TOKEN_ADMIN_LABEL).await?; let price_changer =
        // PriceChanger::new(&environment, &token_admin, &config).await?;
        let mut agents = Agents::new();
        agents.add(block_admin);
        let steps = 10;

        builder.agents(agents);
        builder.steps(steps);
        builder.environment(environment);

        let simulation = builder.build().unwrap();

        assert_eq!(simulation.steps, steps);
    }

    #[test]
    fn test_world_builder() {
        let mut builder = WorldBuilder::new();
        let mut simulations = HashMap::new(); // Replace with your actual initialization
        let simulation = SimulationBuilder::new()
            .agents(Agents::new())
            .steps(10)
            .environment(
                EnvironmentBuilder::new()
                    .block_settings(BlockSettings::UserControlled)
                    .build(),
            )
            .build()
            .unwrap();
        let k = "test".to_string();
        simulations.insert(k.clone(), simulation);
        builder.simulations(simulations);

        let world = builder.build().unwrap();

        assert!(world.simulations.contains_key(&k));
    }
}
