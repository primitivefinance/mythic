use agents::arbitrageur::{self, Arbitrageur};
use agents::liquidity_provider::LiquidityProvider;
use agents::price_changer::PriceChanger;
use agents::weight_changer::WeightChanger;
use anyhow::Result;
use arbiter_core::data_collection::EventLogger;
use arbiter_core::environment::builder::EnvironmentBuilder;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange, environment::Environment, middleware::RevmMiddleware,
};
use ethers::types::{Address, U256};
use settings::params::PriceProcessParameters;
use tracing_subscriber;

use bindings::{atomic_arbitrage::AtomicArbitrage, g3m::G3M};
use setup::init::init;

mod agents;
mod engine;
mod settings;
mod setup;
mod utils;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

#[derive(Clone)]
pub struct Agents {
    pub liquidity_provider: LiquidityProvider,
    pub weight_changer: WeightChanger,
    pub arbitrageur: Arbitrageur<G3M<RevmMiddleware>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = settings::params::SimulationConfig::new()?;

    let env = EnvironmentBuilder::new().build();
    let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

    // have the loop iterate blcoks and block timestamps
    // draw random # from poisson distribution which determines how long we wait for price to change
    // loop that causes price change -> arbitrageur -> check if weightchanger needs to run
    let lp = LiquidityProvider::new("lp", &env, contracts.exchanges.g3m.address()).await?;

    let price_process_params = PriceProcessParameters {
        initial_price: 1.0,
        mean: 1.0,
        std_dev: 0.01,
        theta: 0.5,
        t_0: 0.0,
        t_n: 2.0,
        num_steps: 2,
        seed: Some(1),
    };
    let mut price_changer = PriceChanger::new(
        "price_changer",
        &env,
        contracts.tokens.arbx.address(),
        contracts.tokens.arby.address(),
        price_process_params,
    )
    .await?;

    let lex_address = price_changer.liquid_exchange.address();

    let weight_changer = WeightChanger::new(
        "rebalancer",
        &env,
        lex_address,
        contracts.exchanges.g3m.address(),
        0.15,
    )
    .await?;

    let arbitrageur = Arbitrageur::<G3M<RevmMiddleware>>::new(
        "arbitrageur",
        &env,
        lex_address,
        contracts.exchanges.g3m.address(),
    )
    .await?;

    let mut agents = Agents {
        liquidity_provider: lp,
        weight_changer,
        arbitrageur,
    };

    init(&contracts, &agents, &config).await?;

    EventLogger::builder()
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(contracts.tokens.arbx.events(), "arbx")
        .add(contracts.tokens.arby.events(), "arby")
        .add(contracts.exchanges.g3m.events(), "g3m")
        .run()?;

    for index in 0..price_process_params.num_steps {
        println!("index: {}", index);
        price_changer.update_price().await?;
        agents.arbitrageur.step().await?;
    }

    // engine::builder::EngineBuilder::new()
    //     .run(&contracts, &env, &config)
    //     .await?;

    Ok(())
}
