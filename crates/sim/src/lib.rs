pub mod agent;
pub mod agents;
pub mod config;
pub mod engine;

use anyhow::{Error, Result};
// todo: remove
use simulation::agents::{block_admin::BlockAdminParameters, AgentParameters};
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
