pub mod agent;
pub mod agents;
pub mod config;
pub mod engine;

use std::any::Any;

use anyhow::{Error, Result};
use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use ethers::types::H160;
use serde::{Deserialize, Serialize};
// todo: remove
use simulation::agents::{block_admin::BlockAdminParameters, AgentParameters};
use simulation::settings::{
    parameters::{Multiple, Single},
    Parameterized, SimulationConfig,
};
use tracing_subscriber;

pub fn run() -> Result<()> {
    tracing_subscriber::fmt().init();

    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let result = rt.block_on(async move {
        let mut manager = engine::ArbiterInstanceManager::new();
        // todo: figure out the keys for the agent parameters, as if they are not found
        // it panics.
        manager.config_builder.config.agent_parameters.insert(
            "block_admin".to_string(),
            AgentParameters::BlockAdmin(BlockAdminParameters { timestep_size: 9 }),
        );

        // Run the sims, returning snapshot dbs to the manager's `instances`.
        manager.run_parallel().await
    })?;

    tracing::info!("Simulation result: {:?}", result);

    Ok(())
}

pub fn from_ethers_address(address: H160) -> alloy_primitives::Address {
    alloy_primitives::Address::from(address.as_fixed_bytes())
}

pub fn from_ethers_u256(value: ethers::types::U256) -> alloy_primitives::U256 {
    alloy_primitives::U256::from(value.as_u128())
}

pub fn to_ethers_u256(value: alloy_primitives::U256) -> ethers::types::U256 {
    ethers::types::U256::from_str_radix(value.to_string().as_str(), 10).unwrap()
}
