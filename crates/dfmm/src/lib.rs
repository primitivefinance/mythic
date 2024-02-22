pub mod agent;
pub mod agents;
pub mod configuration;
pub mod engine;
pub mod portfolio;
pub mod rmm;
pub mod scenarios;
pub mod settings;
pub mod position;

use std::{any::Any, path::Path};

use ::config::ConfigError;
use anyhow::{bail, Error, Result};
use arbiter_core::{environment::Environment, middleware::ArbiterMiddleware};
use ethers::{prelude::*, utils::format_ether};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{
    parameters::{Multiple, Single},
    Parameterized, SimulationConfig,
};
use thiserror::Error;
use tracing::{debug, error, trace, warn};

pub const WAD: U256 = U256([1_000_000_000_000_000_000, 0, 0, 0]);
pub const MAX: U256 = U256::MAX;

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
    rt.block_on(async move {
        let mut manager = engine::ArbiterInstanceManager::new();
        manager.config_builder.config = import(path)?;

        manager
            .run_parallel(manager.config_builder.config.scenario.clone())
            .await
    })?;

    let duration = instant.elapsed();

    tracing::info!("Total duration of simulations: {:?}", duration);

    Ok(())
}

#[derive(Debug, Clone)]
enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
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

fn parse_ether_to_f64(ether: ethers::types::U256) -> Result<f64> {
    Ok(format_ether(ether).parse::<f64>()?)
}
