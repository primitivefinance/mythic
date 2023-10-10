use crate::settings::params::SimulationConfig;

use super::deploy::Contracts;
use ethers::types::U256;
use ethers::utils::parse_ether;

// Initial balances of the deployer and portfolio where (arbx, arby)
pub const INITIAL_BALANCES: (u64, u64) = (250, 250);
pub const INITIAL_PORTFOLIO_BALANCES: (u64, u64) = (100, 100);

/// Initialize the pools
/// In initialize we add liquidity and seed actors with tokens
pub async fn init(contracts: &Contracts, config: &SimulationConfig) -> Result<(), anyhow::Error> {
    // Setup the initial token state by minting tokens to the deployer and approving the G3M contract to spend them.
    setup_token_state(&contracts).await?;

    // Setup initial contract state of G3M by initializing the pool.
    setup_pool_state(&contracts, config).await?;

    Ok(())
}

/// All the stateful calls to the tokens used in the simulation.
pub async fn setup_token_state(contracts: &Contracts) -> Result<(), anyhow::Error> {
    // Mint INITIAL_BALANCES of tokens to deployer.
    contracts
        .tokens
        .arbx
        .mint(
            contracts.deployer.address(),
            parse_ether(INITIAL_BALANCES.0).unwrap(),
        )
        .send()
        .await?
        .await?;
    contracts
        .tokens
        .arby
        .mint(
            contracts.deployer.address(),
            parse_ether(INITIAL_BALANCES.1).unwrap(),
        )
        .send()
        .await?
        .await?;

    // Get the parsed amounts for the portfolio deposit.
    let amounts = (
        parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap(),
        parse_ether(INITIAL_PORTFOLIO_BALANCES.1).unwrap(),
    );

    // Approve tokens to be spent by the G3M contract.
    contracts
        .tokens
        .arbx
        .approve(contracts.exchanges.g3m.address(), amounts.0)
        .send()
        .await?
        .await?;
    contracts
        .tokens
        .arby
        .approve(contracts.exchanges.g3m.address(), amounts.1)
        .send()
        .await?
        .await?;

    Ok(())
}

/// Initializes the pool with the desired amounts of tokens, making it have an initial spot price.
pub async fn setup_pool_state(
    contracts: &Contracts,
    config: &SimulationConfig,
) -> Result<(), anyhow::Error> {
    // Initial weight is set in the simulation config, but it can be overridden with setWeightX() function.
    let initial_weight_0 = parse_ether(config.portfolio_pool_parameters.weight_token_0).unwrap();
    let initial_weight_1 = parse_ether(1)
        .unwrap()
        .checked_sub(initial_weight_0)
        .unwrap();
    // Using the initial weight, initial price, and initial reserve x, we can compute reserve y.
    let initial_price = config.portfolio_pool_parameters.initial_price;
    let initial_reserve_x = parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap();

    // p = (x / w_x) / (y / w_y)
    // y / w_y = (x / w_x) / p
    // y = (x / w_x) / p * w_y
    let one_ether = parse_ether(1).unwrap();
    let initial_reserve_y = initial_reserve_x
        .checked_mul(one_ether)
        .unwrap()
        .checked_div(initial_weight_0)
        .unwrap()
        .checked_mul(one_ether)
        .unwrap()
        .checked_div(parse_ether(initial_price).unwrap())
        .unwrap()
        .checked_mul(initial_weight_1)
        .unwrap()
        .checked_div(one_ether)
        .unwrap();

    // Get the parsed amounts for the portfolio deposit.
    let amounts = (initial_reserve_x, initial_reserve_y);

    // Call init pool to setup the portfolio
    // Needs an amount of both tokens, the amounts can be anything but note that they affect the spot price.
    let init_pool = contracts
        .exchanges
        .g3m
        .init_pool(amounts.0.into(), amounts.1.into())
        .send()
        .await?
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings;
    use crate::settings::params::SimulationConfig;
    use crate::setup;
    use arbiter_core::environment::builder::EnvironmentBuilder;
    use arbiter_core::environment::Environment;

    /// Initializes the environment and deploys the contracts before calling the init function.
    async fn setup_test_environment(
    ) -> Result<(Contracts, Environment, SimulationConfig), anyhow::Error> {
        let config = settings::params::SimulationConfig::new()?;

        let env = EnvironmentBuilder::new().build();
        let contracts = setup::deploy::deploy_contracts(&env, &config).await?;

        Ok((contracts, env, config))
    }

    #[tokio::test]
    /// Should return the correct initial token balances of the contracts + agents.
    async fn test_initial_balances() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        // Setup the token state to assert the balances are set and spent correctly.
        setup_token_state(&contracts).await?;

        let balance_0 = contracts
            .tokens
            .arbx
            .balance_of(contracts.deployer.address())
            .call()
            .await?;
        let balance_1 = contracts
            .tokens
            .arby
            .balance_of(contracts.deployer.address())
            .call()
            .await?;

        assert_eq!(balance_0, parse_ether(INITIAL_BALANCES.0).unwrap());
        assert_eq!(balance_1, parse_ether(INITIAL_BALANCES.1).unwrap());

        Ok(())
    }

    #[tokio::test]
    /// Should return the correct constructor arguments of the contracts.
    async fn test_constructor_args() -> Result<(), anyhow::Error> {
        todo!()
    }

    #[tokio::test]
    /// Should return the desired start price of the pool.
    async fn test_initial_pool_price() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;
        init(&contracts, &config).await?;

        let price = contracts.exchanges.g3m.get_spot_price().call().await?;
        println!("Price: {}", price);

        let target_price = parse_ether(config.portfolio_pool_parameters.initial_price).unwrap();
        println!("Target Price: {}", target_price);
        assert_eq!(price, target_price);

        Ok(())
    }

    #[tokio::test]
    /// Should return the correct balances of the pool after initialization.
    async fn test_initial_pool_balances() -> Result<(), anyhow::Error> {
        let (contracts, env, config) = setup_test_environment().await?;

        // Setup the token state to assert the balances are set and spent correctly.
        setup_token_state(&contracts).await?;
        // Setup the pool state to assert the tokens are sent into the pool on initialization.
        setup_pool_state(&contracts, &config).await?;

        let balance_0 = contracts
            .tokens
            .arbx
            .balance_of(contracts.deployer.address())
            .call()
            .await?;
        let balance_1 = contracts
            .tokens
            .arby
            .balance_of(contracts.deployer.address())
            .call()
            .await?;
        let balance_2 = contracts
            .tokens
            .arbx
            .balance_of(contracts.exchanges.g3m.address())
            .call()
            .await?;
        let balance_3 = contracts
            .tokens
            .arby
            .balance_of(contracts.exchanges.g3m.address())
            .call()
            .await?;

        assert_eq!(
            balance_0,
            parse_ether(INITIAL_BALANCES.0 - INITIAL_PORTFOLIO_BALANCES.0).unwrap()
        );
        assert_eq!(
            balance_1,
            parse_ether(INITIAL_BALANCES.1 - INITIAL_PORTFOLIO_BALANCES.1).unwrap()
        );
        assert_eq!(
            balance_2,
            parse_ether(INITIAL_PORTFOLIO_BALANCES.0).unwrap()
        );
        assert_eq!(
            balance_3,
            parse_ether(INITIAL_PORTFOLIO_BALANCES.1).unwrap()
        );

        Ok(())
    }
}
