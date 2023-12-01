use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use arbiter_core::{environment::Environment, middleware::RevmMiddleware};
use ethers::types::Address;
use tracing::info;
use RustQuant::portfolio;

use crate::{
    portfolio_bindings::portfolio::Portfolio,
    settings::{parameters::Single, SimulationConfig},
};

#[derive(Debug)]
pub struct PortfolioPoolInitializer {
    pub client: Arc<RevmMiddleware>,
    pub label: String,
    pub portfolio: Portfolio<RevmMiddleware>,
    pub pair_ids: Vec<u32>,
}

impl PortfolioPoolInitializer {
    pub async fn new(
        environment: &Environment,
        label: impl Into<String>,
        config: &SimulationConfig<Single>,
        portfolio_address: Address,
    ) -> Self {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label)).unwrap();
        let portfolio = Portfolio::new(portfolio_address, client.clone());
        Self {
            client,
            label,
            portfolio,
            pair_ids: Vec::new(),
        }
    }
    pub async fn initialize_pool(&mut self, token_x: Address, token_y: Address) -> Result<()> {
        let mut portfolio = self.portfolio.clone();
        let pool = portfolio
            .create_pair(token_x, token_y)
            .send()
            .await?
            .await?
            .unwrap();
        let pair_id = portfolio.get_pair_id(token_x, token_y).call().await?;
        self.pair_ids.push(pair_id);
        info!("Created a pair with pair_id: {:?}", pair_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl super::Agent for PortfolioPoolInitializer {
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    async fn startup(&mut self) -> Result<()> {
        self.initialize_pool(self.client.address(), self.client.address())
            .await?;
        Ok(())
    }

    /// Executed by each agent inside the main simulation loop.
    /// Ordering is determined by placement in the simulation loop.
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }

    /// Executed by each agent in a separate loop before the main loop.
    async fn priority_step(&mut self) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}
