use std::{any::Any, sync::Arc};

use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use ethers::providers::Middleware;
use revm::db::{CacheDB, EmptyDB};
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::*;
use crate::{
    agent::Agent,
    agents::AgentParameters,
    settings::{parameters::Single, SimulationConfig},
};

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

impl BlockAdmin {
    pub async fn new(
        db: Option<CacheDB<EmptyDB>>,
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label));
        let client = match client {
            Ok(client) => client,
            Err(_) => {
                // If the account already exists in the Environment's db, then we can use the db
                // account instead.
                // todo: figure out how to get the correct address for this account..
                if db.is_none() {
                    return Err(anyhow::anyhow!("No account found in db"));
                }

                let address = db.unwrap().accounts.into_keys().next().unwrap();
                let ethers_address = ethers::types::Address::from(address.into_array());

                RevmMiddleware::new_from_forked_eoa_with_label(
                    environment,
                    ethers_address,
                    Some("block_admin"),
                )?
            }
        };

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        debug!("Updating block");
        self.update_block()?;
        debug!("Block updated");
        Ok(())
    }

    async fn init(&mut self) -> Result<()> {
        debug!("Updating block");
        self.update_block()?;
        debug!("Block updated");
        Ok(())
    }
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
}
