use std::sync::Arc;

use alloy_primitives::{Address, U256};
use arbiter_core::{environment::Environment, errors::ArbiterCoreError};
use ethers::types::Bytes;
use revm::db::{CacheDB, DbAccount, EmptyDB};
use revm_primitives::{AccountInfo, HashMap as Map};
use serde::{Deserialize, Serialize};
use tokio::{runtime::Builder, sync::Semaphore};

use super::*;
use crate::{
    agent::Agents,
    configuration::{ConfigBuilder, Configurable},
    scenarios::Scenario,
    settings::{
        parameters::{Multiple, Single},
        SimulationConfig,
    },
};

/// A live arbiter environment with agents, a config, and amount of steps to
/// run.
#[derive(Debug)]
pub struct ArbiterInstance {
    pub environment: Environment,
    pub config: SimulationConfig<Single>,
    pub agents: Agents,
    pub steps: usize,
}

impl Default for ArbiterInstance {
    fn default() -> Self {
        Self {
            environment: Environment::builder().build(),
            config: SimulationConfig::default(),
            agents: Agents::default(),
            steps: 0,
        }
    }
}

impl ArbiterInstance {
    pub fn new(
        environment: Environment,
        config: SimulationConfig<Single>,
        agents: Agents,
        steps: usize,
    ) -> Self {
        Self {
            environment,
            config,
            agents,
            steps,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        for (_, agent) in self.agents.0.iter_mut() {
            agent.init().await?;
        }

        Ok(())
    }

    pub async fn step(&mut self) -> Result<(), ArbiterCoreError> {
        for (_, agent) in self.agents.0.iter_mut() {
            agent.step().await?;
        }

        Ok(())
    }

    pub async fn exit(&mut self) -> Result<()> {
        for (_, agent) in self.agents.0.iter_mut() {
            agent.exit().await?;
        }

        Ok(())
    }

    /// Consumes this instance, stopping the environment and returning the
    /// snapshot of its db.
    pub fn stop(self) -> Result<()> {
        self.environment.stop()?;
        Ok(())
    }

    pub fn snapshot(db: &CacheDB<EmptyDB>) -> SnapshotDB {
        SnapshotDB::new(db)
    }
}

/// A serializable snapshot of the database used in an Arbiter Instance.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SnapshotDB {
    pub accounts: Map<Address, AccountInfo>,
    pub storage: Map<Address, Map<U256, U256>>,
}

/// Converts the [`db`] returned from an Arbiter Instance into a serializable
/// [`SnapshotDB`].
impl SnapshotDB {
    pub fn new(db: &CacheDB<EmptyDB>) -> Self {
        let accounts = db
            .accounts
            .iter()
            .map(|(k, v)| (Address::from(k.into_array()), v.info.clone()))
            .collect();

        let storage = db
            .accounts
            .iter()
            .map(|(k, v)| {
                let storage = v.storage.iter().map(|(k, v)| (*k, *v)).collect();

                (Address::from(k.into_array()), storage)
            })
            .collect();

        Self { accounts, storage }
    }
}

impl From<SnapshotDB> for CacheDB<EmptyDB> {
    fn from(snapshot: SnapshotDB) -> Self {
        let accounts = snapshot
            .accounts
            .iter()
            .map(|(k, v)| {
                let db_account: DbAccount = v.clone().into();

                (
                    revm_primitives::Address::from(k.into_array()),
                    db_account.clone(),
                )
            })
            .collect();

        // For contracts, loop over the accounts that have non-empty `code` and then
        // load the storage from those into contracts.
        // Collect the (keccak256 hash of code, code).
        let contracts = snapshot
            .accounts
            .iter()
            .filter(|(_, v)| v.code.is_some())
            .map(|(_, v)| {
                let hash = revm_primitives::keccak256(v.code.clone().unwrap().bytes());
                (hash, v.code.clone().unwrap())
            })
            .collect();

        Self {
            accounts,
            contracts,
            ..Default::default()
        }
    }
}

/// A parameterized config (i.e. multiple parameters) and an environment builder
/// that can be used to reconstruct live instances, run them, and then stop
/// them, returning a snapshot of their dbs.
///
/// Config manager is in control of the simulation parameterization.
/// Environment builder is in control of the environment, including the db the
/// environment is started with.
#[derive(Debug, Clone)]
pub struct ArbiterInstanceManager {
    pub instances: Vec<SnapshotDB>,
    pub config_builder: ConfigBuilder,
}

impl Default for ArbiterInstanceManager {
    fn default() -> Self {
        Self {
            instances: vec![],
            config_builder: ConfigBuilder::new(),
        }
    }
}

impl ArbiterInstanceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn override_config(mut self, config: SimulationConfig<Multiple>) -> Self {
        self.config_builder.config = config;
        self
    }

    pub fn add_agent_configuration(mut self, configurable: &dyn Configurable) -> Self {
        self.config_builder.add_configurable(configurable);
        self
    }

    pub async fn build_instance(
        &mut self,
        config: SimulationConfig<Single>,
        scenario: impl Scenario,
    ) -> ArbiterInstance {
        let environment = Environment::builder().with_console_logs().build();
        let (agents, steps, environment) =
            scenario.setup(environment, config.clone()).await.unwrap();
        ArbiterInstance::new(environment, config.clone(), agents, steps)
    }

    pub async fn build(&mut self, scenario: impl Scenario) -> Vec<ArbiterInstance> {
        let configs: Vec<SimulationConfig<Single>> = self.config_builder.get().clone().into();

        let mut instances = vec![];
        for config in configs {
            let instance = self.build_instance(config, scenario.clone()).await;
            instances.push(instance);
        }

        instances
    }

    pub fn stop(&mut self, instances: Vec<ArbiterInstance>) {
        for instance in instances {
            instance.stop().unwrap();
        }
    }

    pub async fn run_parallel(&mut self, scenario: impl Scenario) -> Result<(), Error> {
        let start_time = std::time::Instant::now();
        let result = run_parallel(self.clone(), scenario).await;
        result?.join().unwrap().unwrap();
        let duration = start_time.elapsed();
        tracing::warn!("Simulation tasks finished in {:?}", duration);
        Ok(())
    }
}

impl Serialize for ArbiterInstanceManager {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let saved_dbs: Vec<_> = self.instances.to_vec();
        let config = self.config_builder.clone();
        (saved_dbs, config).serialize(serializer)
    }
}

/// We can't deserialize the instances, but we can deserialize the
/// ArbiterInstanceManager which can build them all.
impl<'de> Deserialize<'de> for ArbiterInstanceManager {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (saved_dbs, config): (Vec<SnapshotDB>, ConfigBuilder) =
            Deserialize::deserialize(deserializer)?;

        // Apply the saved dbs to the environment builders.
        let mut manager = ArbiterInstanceManager::new();
        manager.config_builder = config;
        manager.instances = saved_dbs.clone();

        Ok(manager)
    }
}

type ParallelResult = std::thread::JoinHandle<Result<Vec<()>, Error>>;

pub async fn run_parallel(
    builder: ArbiterInstanceManager,
    scenario: impl Scenario,
) -> Result<ParallelResult, Error> {
    tracing::info!("Running simulation tasks in separate thread.");
    let slice = std::thread::spawn(move || {
        let rt = Builder::new_multi_thread().build()?;
        let max_parallel = builder.config_builder.config.max_parallel;
        let errors = Arc::new(tokio::sync::Mutex::new(vec![] as Vec<Error>));
        let mut builder = builder.clone();
        let semaphore = max_parallel.map(|max| Arc::new(Semaphore::new(max)));
        rt.block_on(async {
            let mut instances = builder.build(scenario).await;
            let mut handles = vec![];
            let i = instances.len();

            for _ in 0..i {
                let instance = instances.remove(0);
                let errors_clone = errors.clone();
                let semaphore_clone = semaphore.clone();
                handles.push(tokio::spawn(
                    simulation_task(instance, semaphore_clone, errors_clone).await,
                ));
            }

            let snapshots = vec![];
            for handle in handles {
                let instance = handle.await???;
                instance.stop()?;
            }

            let mut errors = errors.lock().await;
            if errors.len() > 0 {
                Err(errors.remove(0))
            } else {
                Ok(snapshots)
            }
        })
    });
    Ok(slice)
}

async fn simulation_task(
    instance: ArbiterInstance,
    semaphore: Option<Arc<Semaphore>>,
    errors: Arc<tokio::sync::Mutex<Vec<Error>>>,
) -> tokio::task::JoinHandle<Result<ArbiterInstance, Error>> {
    let errors_clone = errors.clone();
    tokio::spawn(async move {
        let mut instance = instance;
        instance.init().await.unwrap();

        warn!("Running simulation task.");
        if let Some(semaphore) = &semaphore {
            let permit = semaphore.acquire().await.unwrap();
            for i in 0..instance.steps {
                let result: Result<(), ArbiterCoreError> = instance.step().await;

                match result {
                    Err(e) => {
                        match &e {
                            ArbiterCoreError::ExecutionRevert { output, .. } => {
                                tracing::error!(
                                    "Transaction revert at step {} with revert: {:?}",
                                    i,
                                    Bytes::from(output.clone())
                                );
                                let mut errors_clone_lock = errors_clone.lock().await;
                                errors_clone_lock.push(e.into());

                                // Drop the permit when the simulation is done.
                                drop(permit);

                                // Exits the simulation.
                                break;
                            }
                            _ => {
                                tracing::error!("Error at step {} with message: {:?}", i, e);
                                let mut errors_clone_lock = errors_clone.lock().await;
                                errors_clone_lock.push(e.into());

                                // Drop the permit when the simulation is done.
                                drop(permit);

                                // Exits the simulation.
                                break;
                            }
                        };
                    }
                    Ok(_) => {
                        // Continue running the simulation.
                    }
                }
            }
        } else {
            for i in 0..instance.steps {
                let result: Result<(), ArbiterCoreError> = instance.step().await;

                match result {
                    Err(e) => {
                        tracing::error!(
                            "Simulation got an error after calling `step` on step {} {:?}",
                            i,
                            e
                        );

                        let mut errors_clone_lock = errors_clone.lock().await;
                        errors_clone_lock.push(e.into());

                        // Exits the simulation.
                        break;
                    }
                    Ok(_) => {
                        // Continue running the simulation.
                    }
                }
            }
        }

        instance.exit().await.unwrap();

        Ok(instance)
    })
}
