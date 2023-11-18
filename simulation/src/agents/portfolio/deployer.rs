use anyhow::Ok;
use arbiter_core::bindings::weth::WETH;

use super::*;
use crate::portfolio_bindings::portfolio::Portfolio;

#[derive(Debug)]
pub struct PortfolioDeployer {
    pub client: Arc<RevmMiddleware>,
    pub portfolio: Portfolio<RevmMiddleware>,
    pub weth: WETH<RevmMiddleware>,
    pub label: String,
}

impl PortfolioDeployer {
    pub async fn new(environment: &Environment, label: impl Into<String>) -> Result<Self> {
        println!("new PortfolioDeployer");
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        println!("client address: {:?}", client.address());
        let weth = WETH::deploy(client.clone(), ())?.send().await?;
        println!("weth address: {:?}", weth.address());
        let portfolio = Portfolio::deploy(
            client.clone(),
            (weth.address(), client.address(), client.address()),
        )?
        .send()
        .await?;
        println!("portfolio address: {:?}", portfolio.address());
        Ok(Self {
            client,
            portfolio,
            weth,
            label,
        })
    }
}

#[async_trait::async_trait]
impl Agent for PortfolioDeployer {
    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    async fn startup(&mut self) -> Result<()> {
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

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}
