use std::sync::Arc;

use alloy_primitives::{Address, U256};
use arbiter_core::environment::{
    builder::{BlockSettings, EnvironmentBuilder, GasSettings},
    Environment,
};
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
            environment: EnvironmentBuilder::new().build(),
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

    pub async fn step(&mut self) -> Result<()> {
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
    pub fn stop(self) -> Result<SnapshotDB> {
        let db = self.environment.stop()?;
        Ok(Self::snapshot(&db.clone().unwrap()))
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
            .map(|(k, v)| (Address::from(k.clone().into_array()), v.info.clone()))
            .collect();

        let storage = db
            .accounts
            .iter()
            .map(|(k, v)| {
                let storage = v
                    .storage
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                (Address::from(k.clone().into_array()), storage)
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
                    revm_primitives::Address::from(k.clone().into_array()),
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
    pub builder: EnvironmentBuilder,
    pub config_builder: ConfigBuilder,
}

impl Default for ArbiterInstanceManager {
    fn default() -> Self {
        Self {
            instances: vec![],
            builder: EnvironmentBuilder::new(),
            config_builder: ConfigBuilder::new(),
        }
    }
}

impl ArbiterInstanceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_settings(mut self, block_settings: BlockSettings) -> Self {
        self.builder = self.builder.block_settings(block_settings);
        self
    }

    pub fn gas_settings(mut self, gas_settings: GasSettings) -> Self {
        self.builder = self.builder.gas_settings(gas_settings);
        self
    }

    pub fn db(mut self, db: CacheDB<EmptyDB>) -> Self {
        self.builder = self.builder.db(db);
        self
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
        let db = self.builder.db.clone();
        let environment = self.builder.clone().build();
        let (agents, steps, environment) = scenario
            .setup(db, environment, config.clone())
            .await
            .unwrap();
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
            let db = instance.stop().unwrap();
            self.instances.push(db);
        }
    }

    pub async fn run_parallel(
        &mut self,
        scenario: impl Scenario,
    ) -> Result<Vec<SnapshotDB>, Error> {
        let start_time = std::time::Instant::now();
        let result = run_parallel(self.clone(), scenario).await;
        let result = result?.join().unwrap().unwrap();
        self.instances = result.clone();
        let duration = start_time.elapsed();
        tracing::warn!("Simulation tasks finished in {:?}", duration);
        Ok(result)
    }

    pub fn load_from_snapshot(mut self, snapshot: SnapshotDB) -> Self {
        self.builder = self.builder.db(CacheDB::from(snapshot));
        self
    }
}

impl Serialize for ArbiterInstanceManager {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let saved_dbs: Vec<_> = self.instances.iter().map(|db| db.clone()).collect();
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

type ParallelResult = std::thread::JoinHandle<Result<Vec<SnapshotDB>, Error>>;

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
        let semaphore = match max_parallel {
            Some(max) => Some(Arc::new(Semaphore::new(max))),
            None => None,
        };
        let res = rt.block_on(async {
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

            let mut snapshots = vec![];
            for handle in handles {
                let instance = handle.await???;
                let snapshot = instance.stop()?;
                snapshots.push(snapshot);
            }

            let mut errors = errors.lock().await;
            if errors.len() > 0 {
                Err(errors.remove(0))
            } else {
                Ok(snapshots)
            }
        });
        res
    });
    Ok(slice)
}

async fn simulation_task(
    instance: ArbiterInstance,
    semaphore: Option<Arc<Semaphore>>,
    errors: Arc<tokio::sync::Mutex<Vec<Error>>>,
) -> tokio::task::JoinHandle<Result<ArbiterInstance, Error>> {
    let errors_clone = errors.clone();
    let handle = tokio::spawn(async move {
        let mut instance = instance;
        instance.init().await.unwrap();

        warn!("Running simulation task.");
        if let Some(semaphore) = &semaphore {
            let permit = semaphore.acquire().await.unwrap();
            for i in 0..instance.steps {
                let result: Result<(), Error> = instance.step().await;

                match result {
                    Err(e) => {
                        tracing::error!(
                            "Simulation got an error after calling `step` on step {} {:?}",
                            i,
                            e
                        );

                        let mut errors_clone_lock = errors_clone.lock().await;
                        errors_clone_lock.push(e);

                        // Drop the permit when the simulation is done.
                        drop(permit);

                        // Exits the simulation.
                        break;
                    }
                    Ok(_) => {
                        // Continue running the simulation.
                    }
                }
            }
        } else {
            for i in 0..instance.steps {
                let result: Result<(), Error> = instance.step().await;

                match result {
                    Err(e) => {
                        tracing::error!(
                            "Simulation got an error after calling `step` on step {} {:?}",
                            i,
                            e
                        );

                        let mut errors_clone_lock = errors_clone.lock().await;
                        errors_clone_lock.push(e);

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
    });

    handle
}

#[cfg(test)]
mod tests {
    use arbiter_core::{environment::cheatcodes, middleware::RevmMiddleware};

    use super::*;
    use crate::{
        agents::{base::block_admin::BlockAdminParameters, AgentParameters},
        scenarios::BasicScenario,
    };

    #[tokio::test]
    async fn test_serialize_deserialize() {
        // Start the tracing.
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();

        let scenario = BasicScenario {};

        // Make a new manager and add an agent parameter to it.
        let mut manager = ArbiterInstanceManager::new();

        // todo: figure out the keys for the agent parameters, as if they are not found
        // it panics.
        manager.config_builder.config.agent_parameters.insert(
            "block_admin".to_string(),
            AgentParameters::BlockAdmin(BlockAdminParameters { timestep_size: 9 }),
        );

        // Run the sims, returning snapshot dbs to the manager's `instances`.
        manager.run_parallel(scenario.clone()).await.unwrap();

        // Serializing the manager will serialize its builders and snapshot dbs.
        let serialized = serde_json::to_string(&manager).unwrap();
        let mut deserialized: ArbiterInstanceManager = serde_json::from_str(&serialized).unwrap();

        // Load the snapshot into the environment builder, so the built environments use
        // it.
        let snapshot = deserialized.instances.first().cloned().unwrap();
        deserialized = deserialized.load_from_snapshot(snapshot);

        // Build the instances.
        let instance = deserialized.build(scenario).await;

        // Only care about the first instance, in which we added the block admin.
        let instance = instance.first().unwrap();
        let client = RevmMiddleware::new(&instance.environment, Some("test")).unwrap();

        // Try getting the block admin from the instance, which was loaded from the
        // deserialized snapshot db.
        let block_admin_address = instance
            .agents
            .0
            .values()
            .next()
            .unwrap()
            .client()
            .address()
            .clone();
        let address =
            revm_primitives::alloy_primitives::Address::from(block_admin_address.as_fixed_bytes());
        let account = client
            .apply_cheatcode(cheatcodes::Cheatcodes::Access { address })
            .await
            .unwrap();

        let account = match account {
            cheatcodes::CheatcodesReturn::Access { info, .. } => Some(info),
            _ => None,
        };

        assert_eq!(instance.config.agent_parameters.len(), 1);
        assert_eq!(account.is_some(), true);
    }
}
