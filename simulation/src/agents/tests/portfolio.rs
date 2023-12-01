use arbiter_core::environment::builder::{BlockSettings, EnvironmentBuilder};

use crate::{
    agents::{
        portfolio::{deployer::PortfolioDeployer, initializer::PortfolioPoolInitializer},
        token_admin::TokenAdmin,
    },
    settings::{parameters::Single, SimulationConfig},
    simulations,
};

const CONFIG_PATH: &str = "src/tests/configs/static.toml";

#[tokio::test]
async fn test_portfolio_deployer() {
    let label = "test_label".to_string();
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .label("env_label".to_string())
        .build();

    let deployer = PortfolioDeployer::new(&environment, label).await;

    assert!(deployer.is_ok());
    let deployer = deployer.unwrap();
    assert_eq!(deployer.label, "test_label");
}

#[tokio::test]
async fn portfolio_pool_initializer() {
    let deployer_label = "deployer".to_string();
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .label("env_label".to_string())
        .build();
    let config: Vec<SimulationConfig<Single>> = simulations::import(CONFIG_PATH).unwrap().into();
    let deployer = PortfolioDeployer::new(&environment, &deployer_label)
        .await
        .unwrap();

    // let single_config = config;

    let portfolio_address = deployer.portfolio.address();

    let token_admin_label = "token_admin".to_string();

    let token_admin = TokenAdmin::new(&environment, &config[0].clone(), &token_admin_label)
        .await
        .unwrap();

    let token_x = token_admin.arbx.address();
    let token_y = token_admin.arby.address();

    let initializer_label = "initializer".to_string();

    let mut initializer = PortfolioPoolInitializer::new(
        &environment,
        &initializer_label,
        &config[0],
        portfolio_address,
    )
    .await;
}
