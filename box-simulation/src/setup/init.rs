use crate::settings::params::SimulationConfig;
use crate::Agents;

use super::deploy::Contracts;
use ethers::utils::parse_ether;

// Initial balances of the deployer and portfolio where (arbx, arby)
pub const INITIAL_PORTFOLIO_BALANCES: (u64, u64) = (100_000, 100_000);

/// Initialize the pools
/// In initialize we add liquidity and seed actors with tokens
/// All the stateful calls to the tokens used in the simulation.
pub async fn init(
    contracts: &Contracts,
    agents: &Agents,
    config: &SimulationConfig,
) -> Result<(), anyhow::Error> {
    let amounts = (
        parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap(),
        parse_ether(INITIAL_PORTFOLIO_BALANCES.1).unwrap(),
    );

    let agent_addrs = vec![
        agents.liquidity_provider.client.address(),
        agents.weight_changer.client.address(),
        agents.arbitrageur.client.address(),
    ];

    for addr in agent_addrs {
        // Mint INITIAL_PORTFOLIO_BALANCES of tokens to agents.
        contracts
            .tokens
            .arbx
            .mint(addr, amounts.0)
            .send()
            .await?
            .await?;
        contracts
            .tokens
            .arby
            .mint(addr, amounts.1)
            .send()
            .await?
            .await?;
    }

    agents
        .clone()
        .liquidity_provider
        .add_liquidity(config)
        .await?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::settings;
//     use crate::settings::params::SimulationConfig;
//     use crate::setup;
//     use arbiter_core::environment::builder::EnvironmentBuilder;
//     use arbiter_core::environment::Environment;
//     use arbiter_core::middleware::RevmMiddleware;
//     use bindings::g3m::G3M;

//     /// Initializes the environment and deploys the contracts before calling the init function.
//     async fn setup_test_environment(
//     ) -> Result<(Contracts, Environment, SimulationConfig), anyhow::Error> {
//         let config = settings::params::SimulationConfig::new()?;

//         let env = EnvironmentBuilder::new().build();
//         let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

//         Ok((contracts, env, config))
//     }

//     #[tokio::test]
//     /// Should return the correct initial token balances of the contracts + agents.
//     async fn test_initial_balances() -> Result<(), anyhow::Error> {
//         let (contracts, env, config) = setup_test_environment().await?;
//         let agents = Agents {
//             liquidity_provider: crate::agents::liquidity_provider::LiquidityProvider::new(
//                 "lp",
//                 &env,
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//             weight_changer: crate::agents::weight_changer::WeightChanger::new(
//                 "rebalancer",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//                 0.15,
//             )
//             .await?,
//             arbitrageur: crate::agents::arbitrageur::Arbitrageur::<G3M<RevmMiddleware>>::new(
//                 "arbitrageur",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//         };

//         // Setup the token state to assert the balances are set and spent correctly.
//         init(&contracts, agents, &config).await?;

//         let balance_0 = contracts
//             .tokens
//             .arbx
//             .balance_of(contracts.exchanges.g3m.address())
//             .call()
//             .await?;
//         let balance_1 = contracts
//             .tokens
//             .arby
//             .balance_of(contracts.exchanges.g3m.address())
//             .call()
//             .await?;

//         assert_eq!(
//             balance_0,
//             parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap()
//         );
//         assert_eq!(
//             balance_1,
//             parse_ether(INITIAL_PORTFOLIO_BALANCES.1).unwrap()
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     /// Should return the correct constructor arguments of the contracts.
//     /// Since the constructor arguments are derived from the simulation config,
//     /// this is a good test for making sure the config is integrated correctly.
//     async fn test_constructor_args() -> Result<(), anyhow::Error> {
//         use ethers::types::U256;
//         let (contracts, env, config) = setup_test_environment().await?;
//         let agents = Agents {
//             liquidity_provider: crate::agents::liquidity_provider::LiquidityProvider::new(
//                 "lp",
//                 &env,
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//             weight_changer: crate::agents::weight_changer::WeightChanger::new(
//                 "rebalancer",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//                 0.15,
//             )
//             .await?,
//             arbitrageur: crate::agents::arbitrageur::Arbitrageur::<G3M<RevmMiddleware>>::new(
//                 "arbitrageur",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//         };
//         init(&contracts, agents, &config).await?;

//         let initial_weight_x_wad = parse_ether(config.portfolio_pool_parameters.weight_token_0)?;
//         let initial_swap_fee_bps = U256::from(config.portfolio_pool_parameters.fee_basis_points);
//         let initial_admin = contracts.deployer.address();
//         let initial_token_x = contracts.tokens.arbx.address();
//         let initial_token_y = contracts.tokens.arby.address();

//         let actual_token_x = contracts.exchanges.g3m.token_x().call().await?;
//         let actual_token_y = contracts.exchanges.g3m.token_y().call().await?;
//         let actual_weight_x = contracts.exchanges.g3m.weight_x().call().await?;
//         let actual_weight_y = contracts.exchanges.g3m.weight_y().call().await?;
//         let actual_swap_fee_bps = contracts.exchanges.g3m.swap_fee().call().await?;
//         let actual_admin = contracts.exchanges.g3m.admin().call().await?;

//         assert_eq!(actual_token_x, initial_token_x);
//         assert_eq!(actual_token_y, initial_token_y);
//         assert_eq!(actual_weight_x, initial_weight_x_wad);
//         assert_eq!(actual_weight_y, parse_ether(1)? - initial_weight_x_wad);
//         assert_eq!(actual_swap_fee_bps, initial_swap_fee_bps);
//         assert_eq!(actual_admin, initial_admin);

//         Ok(())
//     }

//     #[tokio::test]
//     /// Should return the desired start price of the pool.
//     async fn test_initial_pool_price() -> Result<(), anyhow::Error> {
//         let (contracts, env, config) = setup_test_environment().await?;
//         let agents = Agents {
//             liquidity_provider: crate::agents::liquidity_provider::LiquidityProvider::new(
//                 "lp",
//                 &env,
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//             weight_changer: crate::agents::weight_changer::WeightChanger::new(
//                 "rebalancer",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//                 0.15,
//             )
//             .await?,
//             arbitrageur: crate::agents::arbitrageur::Arbitrageur::<G3M<RevmMiddleware>>::new(
//                 "arbitrageur",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//         };
//         init(&contracts, agents, &config).await?;

//         let price = contracts.exchanges.g3m.get_spot_price().call().await?;
//         println!("Price: {}", price);

//         let target_price = parse_ether(config.portfolio_pool_parameters.initial_price).unwrap();
//         println!("Target Price: {}", target_price);
//         assert_eq!(price, target_price);

//         Ok(())
//     }

//     #[tokio::test]
//     /// Should return the correct balances of the pool after initialization.
//     async fn test_initial_pool_balances() -> Result<(), anyhow::Error> {
//         let (contracts, env, config) = setup_test_environment().await?;
//         let agents = Agents {
//             liquidity_provider: crate::agents::liquidity_provider::LiquidityProvider::new(
//                 "lp",
//                 &env,
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//             weight_changer: crate::agents::weight_changer::WeightChanger::new(
//                 "rebalancer",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//                 0.15,
//             )
//             .await?,
//             arbitrageur: crate::agents::arbitrageur::Arbitrageur::<G3M<RevmMiddleware>>::new(
//                 "arbitrageur",
//                 &env,
//                 contracts.exchanges.lex.address(),
//                 contracts.exchanges.g3m.address(),
//             )
//             .await?,
//         };

//         // Setup the token state to assert the balances are set and spent correctly.
//         init(&contracts, agents, &config).await?;

//         let balance_2 = contracts
//             .tokens
//             .arbx
//             .balance_of(contracts.exchanges.g3m.address())
//             .call()
//             .await?;
//         let balance_3 = contracts
//             .tokens
//             .arby
//             .balance_of(contracts.exchanges.g3m.address())
//             .call()
//             .await?;

//         assert_eq!(
//             balance_2,
//             parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap()
//         );
//         assert_eq!(
//             balance_3,
//             parse_ether(INITIAL_PORTFOLIO_BALANCES.1).unwrap()
//         );

//         Ok(())
//     }
// }
