use std::sync::Arc;

use anyhow::Result;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange, environment::Environment, middleware::RevmMiddleware,
};
use ethers::types::{Address, U256};

mod agents;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

/// Highest level simulation loop.
async fn run_sim_loop() -> Result<()> {
    for index in 1..100 {
        // do sim
    }

    Ok(())
}
