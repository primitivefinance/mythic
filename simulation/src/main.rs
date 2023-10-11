use agents::arbitrageur::Arbitrageur;
use agents::liquidity_provider::LiquidityProvider;
use agents::weight_changer::WeightChanger;
use anyhow::Result;
use arbiter_core::environment::builder::EnvironmentBuilder;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange, environment::Environment, middleware::RevmMiddleware,
};
use ethers::types::{Address, U256};
use tracing_subscriber;

use bindings::{atomic_arbitrage::AtomicArbitrage, g3m::G3M};
use setup::init::init;

mod agents;
mod engine;
mod settings;
mod setup;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

pub struct Agents {
    pub liquidity_provider: LiquidityProvider,
    pub rebalancer: WeightChanger,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = settings::params::SimulationConfig::new()?;

    let env = EnvironmentBuilder::new().build();
    let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

    let _arbitrageur = Arbitrageur::<G3M<RevmMiddleware>>::new(
        "arbitrageur",
        &env,
        contracts.exchanges.lex.address(),
        contracts.exchanges.g3m.address(),
    )
    .await?;

    // have the loop iterate blcoks and block timestamps
    // draw random # from poisson distribution which determines how long we wait for price to change
    // loop that causes price change -> arbitrageur -> check if weightchanger needs to run
    let lp = LiquidityProvider::new("lp", &env, contracts.exchanges.g3m.address()).await?;
    let rebalancer = WeightChanger::new(
        "rebalancer",
        &env,
        contracts.exchanges.lex.address(),
        contracts.exchanges.g3m.address(),
        0.15,
    )
    .await?;

    let agents = Agents {
        liquidity_provider: lp,
        rebalancer,
    };

    init(&contracts, agents, &config).await?;

    engine::builder::EngineBuilder::new()
        .run(&contracts, &env, &config)
        .await?;

    Ok(())
}
