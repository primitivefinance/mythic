pub mod agent;
pub mod agents;
pub mod configuration;
pub mod engine;
pub mod scenarios;
pub mod settings;

use std::{any::Any, path::Path};

use ::config::ConfigError;
use anyhow::{Error, Result};
use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use bindings;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{
    parameters::{Multiple, Single},
    Parameterized, SimulationConfig,
};
use thiserror::Error;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

pub fn import(config_path: &str) -> Result<SimulationConfig<Multiple>, ConfigError> {
    let cwd = std::env::current_dir().unwrap();
    let path = Path::new(cwd.to_str().unwrap());
    let path = path.join(config_path);
    println!("Reading config from: {:?}", path);
    SimulationConfig::new(config_path)
}

pub fn run(path: &str, verbosity: Option<u8>) -> Result<()> {
    let log_level = match verbosity.unwrap_or(0) {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    // let config: SimulationConfig<Multiple> = import(path)?;
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let instant = std::time::Instant::now();

    // Run the sims, returning snapshot dbs to the manager's `instances`.
    let result = rt.block_on(async move {
        let mut manager = engine::ArbiterInstanceManager::new();
        manager.config_builder.config = import(path)?;
        let scenario = scenarios::DFMMScenario;
        manager.run_parallel(scenario).await
    })?;

    let duration = instant.elapsed();

    tracing::trace!("Simulation result: {:?}", result);
    tracing::info!("Total duration of simulations: {:?}", duration);

    Ok(())
}

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
