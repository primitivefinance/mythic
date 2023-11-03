use std::sync::Arc;

use arbiter_core::bindings::arbiter_token::ArbiterToken;
use ethers::providers::Middleware;

use super::*;

#[derive(Clone)]
pub struct BlockAdmin {
    pub client: Arc<RevmMiddleware>,
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
        let client = RevmMiddleware::new(environment, Some(&label))?;
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
    async fn step(&mut self) -> Result<()> {
        self.update_block()?;
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        self.update_block()?;
        Ok(())
    }
}
