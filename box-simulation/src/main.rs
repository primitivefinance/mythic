use agents::arbitrageur::Arbitrageur;
use agents::liquidity_provider::LiquidityProvider;
use agents::price_changer::PriceChanger;
use agents::token_admin;
use agents::weight_changer::WeightChanger;
use anyhow::Result;
use arbiter_core::data_collection::EventLogger;
use arbiter_core::environment::builder::EnvironmentBuilder;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange, environment::Environment, middleware::RevmMiddleware,
};
use bindings::g3m::G3M;
use ethers::{
    types::{Address, U256},
    utils::format_ether,
};

mod agents;
mod params;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = params::SimulationConfig::new()?;

    let env = EnvironmentBuilder::new().build();

    let token_admin = token_admin::TokenAdmin::new(&env).await?;
    let mut price_changer =
        PriceChanger::new(&env, &token_admin, config.price_process_parameters).await?;
    let weight_changer = WeightChanger::new(
        &env,
        &config,
        price_changer.liquid_exchange.address(),
        token_admin.arbx.address(),
        token_admin.arby.address(),
    )
    .await?;
    let lp = LiquidityProvider::new(&env, &token_admin, weight_changer.g3m.address()).await?;
    let arbitrageur = Arbitrageur::<G3M<RevmMiddleware>>::new(
        &env,
        &token_admin,
        weight_changer.lex.address(),
        weight_changer.g3m.address(),
    )
    .await?;

    lp.add_liquidity(&config).await?;

    // have the loop iterate blcoks and block timestamps
    // draw random # from poisson distribution which determines how long we wait for price to change
    // loop that causes price change -> arbitrageur -> check if weightchanger needs to run

    EventLogger::builder()
        .add(price_changer.liquid_exchange.events(), "lex")
        .add(token_admin.arbx.events(), "arbx")
        .add(token_admin.arby.events(), "arby")
        .add(weight_changer.g3m.events(), "g3m")
        .run()?;

    for index in 0..config.price_process_parameters.num_steps {
        println!("index: {}", index);
        let init_price = weight_changer.g3m.get_spot_price().call().await?;
        println!(
            "init price: {}",
            format_ether(init_price).parse::<f64>().unwrap()
        );
        price_changer.update_price().await?;
        arbitrageur.step().await?;
        let new_price = weight_changer.g3m.get_spot_price().call().await?;
        println!(
            "new price: {}",
            format_ether(new_price).parse::<f64>().unwrap()
        );
    }

    Ok(())
}
