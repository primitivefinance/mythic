pub mod agent;
pub mod agents;
pub mod bindings;
pub mod config;
pub mod engine;
pub mod scenarios;

use std::{any::Any, path::Path};

use ::config::ConfigError;
use anyhow::{Error, Result};
use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use serde::{Deserialize, Serialize};
// todo: remove
use simulation::agents::AgentParameters;
use simulation::settings::{
    parameters::{Multiple, Single},
    Parameterized, SimulationConfig,
};
use tracing::Level;
use tracing_subscriber;

pub fn import(config_path: &str) -> Result<SimulationConfig<Multiple>, ConfigError> {
    let cwd = std::env::current_dir().unwrap();
    let path = Path::new(cwd.to_str().unwrap());
    let path = path.join(config_path);
    println!("Reading config from: {:?}", path);
    SimulationConfig::new(config_path)
}

pub fn run() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let path = "configs/volatility_targeting/static.toml";
    let config: SimulationConfig<Multiple> = import(path)?;

    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    let instant = std::time::Instant::now();

    // Run the sims, returning snapshot dbs to the manager's `instances`.
    let result = rt.block_on(async move {
        let config = config;
        let mut manager = engine::ArbiterInstanceManager::new();
        manager.config_builder.config = config;
        let scenario = scenarios::DFMMScenario;
        manager.run_parallel(scenario).await
    })?;

    let duration = instant.elapsed();

    tracing::debug!("Simulation result: {:?}", result);
    tracing::info!("Total duration of simulations: {:?}", duration);

    Ok(())
}

pub fn to_ethers_address(address: alloy_primitives::Address) -> ethers::types::Address {
    ethers::types::Address::from(address.into_array())
}

pub fn from_ethers_address(address: ethers::types::Address) -> alloy_primitives::Address {
    alloy_primitives::Address::from(address.as_fixed_bytes())
}

pub fn from_ethers_u256(value: ethers::types::U256) -> alloy_primitives::U256 {
    alloy_primitives::U256::from(value.as_u128())
}

pub fn to_ethers_u256(value: alloy_primitives::U256) -> ethers::types::U256 {
    ethers::types::U256::from_str_radix(value.to_string().as_str(), 10).unwrap()
}
