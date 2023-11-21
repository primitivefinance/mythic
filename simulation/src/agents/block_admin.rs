use std::sync::Arc;

use arbiter_core::bindings::arbiter_token::ArbiterToken;
use ethers::providers::Middleware;

use super::*;

#[derive(Clone, Debug)]
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
pub struct BlockAdminParametersBuilder {
    timestep_size: Option<u64>,
}

impl Default for BlockAdminParametersBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl BlockAdminParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timestep_size(mut self, timestep_size: u64) -> Self {
        self.timestep_size = Some(timestep_size);
        self
    }

    pub fn build(self) -> Result<BlockAdminParameters, &'static str> {
        if self.timestep_size.is_none() {
            Err("Not all fields set")
        } else {
            Ok(BlockAdminParameters {
                timestep_size: self.timestep_size.unwrap(),
            })
        }
    }
}

impl BlockAdmin {
    pub async fn new(
        environment: &Environment,
        block_admin_parameters: BlockAdminParameters,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        Ok(Self {
            client: client.clone(),
            timestep_size: block_admin_parameters.timestep_size,
            block_number: client.get_block_number().await?.as_u64(),
            block_timestamp: client.get_block_timestamp().await?.as_u64(),
        })
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        debug!("Updating block");
        self.update_block()?;
        debug!("Block updated");
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        debug!("Updating block");
        self.update_block()?;
        debug!("Block updated");
        Ok(())
    }
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
}
