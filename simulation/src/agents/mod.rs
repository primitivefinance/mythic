use super::*;

pub mod arbitrageur;
pub mod block_admin;
pub mod liquidity_provider;
pub mod price_changer;
pub mod token_admin;
pub mod weight_changer;

use std::marker::{Send, Sync};

/// Universal agent methods for interacting with the simulation environment or loop.
#[async_trait::async_trait]
pub trait Agent: Sync + Send {
    /// Executed outside the main simulation loop.
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
}
