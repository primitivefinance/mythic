use super::deploy::Contracts;
use ethers::types::U256;
use ethers::utils::parse_ether;

// Initial balances of the deployer and portfolio where (arbx, arby)
pub const INITIAL_BALANCES: (u64, u64) = (250, 250);
pub const INITIAL_PORTFOLIO_BALANCES: (u64, u64) = (100, 100);

/// Initialize the pools
/// In initialize we add liquidity and seed actors with tokens
pub async fn init(contracts: &Contracts) -> Result<(), anyhow::Error> {
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
    use crate::setup;
    use arbiter_core::environment::builder::EnvironmentBuilder;
    use arbiter_core::environment::Environment;

    /// Initializes the environment and deploys the contracts before calling the init function.
    async fn setup_tests() -> Result<(Contracts, Environment), anyhow::Error> {
        let config = settings::params::SimulationConfig::new()?;

        let env = EnvironmentBuilder::new().build();
        let contracts = setup::deploy::deploy_contracts(&env).await?;

        let _ = init(&contracts).await?;

        Ok((contracts, env))
    }

    #[tokio::test]
    /// Should return the correct initial token balances of the contracts + agents.
    async fn test_initial_balances() -> Result<(), anyhow::Error> {
        let (contracts, env) = setup_tests().await?;

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

        assert_eq!(
            balance_0,
            parse_ether(INITIAL_BALANCES.0 - INITIAL_PORTFOLIO_BALANCES.0).unwrap()
        );
        assert_eq!(
            balance_1,
            parse_ether(INITIAL_BALANCES.1 - INITIAL_PORTFOLIO_BALANCES.1).unwrap()
        );

        Ok(())
    }

    #[tokio::test]
    /// Should return the correct constructor arguments of the contracts.
    async fn test_constructor_args() {}

    #[tokio::test]
    /// Should return the desired start price of the pool.
    async fn test_initial_pool_price() {}
}
