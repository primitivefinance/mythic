use std::{any::Any, sync::Arc};

use arbiter_core::{environment::Environment, middleware::ArbiterMiddleware};
use ethers::providers::Middleware;
use revm::db::{CacheDB, EmptyDB};
use serde::{Deserialize, Serialize};

use super::*;
use crate::{
    agent::Agent,
    agents::AgentParameters,
    settings::{parameters::Single, SimulationConfig},
};

#[derive(Clone, Debug)]
pub struct BlockAdmin {
    pub client: Arc<ArbiterMiddleware>,
    pub timestep_size: u64,
    pub block_timestamp: u64,
    pub block_number: u64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BlockAdminParameters {
    pub timestep_size: u64,
}

impl BlockAdmin {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = ArbiterMiddleware::new(environment, Some(&label))?;
        if let Some(AgentParameters::BlockAdmin(parameters)) = config.agent_parameters.get(&label) {
            Ok(Self {
                client: client.clone(),
                timestep_size: parameters.timestep_size,
                block_number: client.get_block_number().await?.as_u64(),
                block_timestamp: client.get_block_timestamp().await?.as_u64(),
            })
        } else {
            Err(anyhow::anyhow!("No parameters found for block admin"))
        }
    }

    pub fn update_block(&mut self) -> Result<()> {
        self.block_number += 1;
        self.block_timestamp = self.block_number * self.timestep_size;
        self.client
            .update_block(self.block_number, self.block_timestamp)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for BlockAdmin {
    async fn init(&mut self) -> Result<()> {
        trace!("Updating block");
        self.update_block()?;
        trace!("Block updated");
        Ok(())
    }

    async fn step(&mut self) -> Result<()> {
        trace!("Updating block");
        self.update_block()?;
        trace!("Block updated");
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn client(&self) -> Arc<ArbiterMiddleware> {
        self.client.clone()
    }
}
