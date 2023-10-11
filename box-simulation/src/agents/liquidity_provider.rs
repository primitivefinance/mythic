use crate::settings::params::SimulationConfig;
use arbiter_core::bindings::arbiter_token::ArbiterToken;
use ethers::utils::parse_ether;
use std::sync::Arc;

use super::*;

pub const INITIAL_PORTFOLIO_BALANCES: (u64, u64) = (100_000, 100_000);

#[derive(Clone)]
pub struct LiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
}

impl LiquidityProvider {
    pub async fn new(label: &str, environment: &Environment, g3m_address: Address) -> Result<Self> {
        let client = RevmMiddleware::new(environment, Some(label))?;
        let g3m = G3M::new(g3m_address, client.clone());

        let arbx = ArbiterToken::new(g3m.token_x().call().await?, client.clone());
        let arby = ArbiterToken::new(g3m.token_y().call().await?, client.clone());

        arbx.approve(g3m.address(), U256::MAX).send().await?;
        arby.approve(g3m.address(), U256::MAX).send().await?;

        Ok(Self {
            client,
            g3m,
            arbx,
            arby,
        })
    }

    pub async fn add_liquidity(self, config: &SimulationConfig) -> Result<()> {
        // Initial weight is set in the simulation config, but it can be overridden with setWeightX() function.
        let initial_weight_0 =
            parse_ether(config.portfolio_pool_parameters.weight_token_0).unwrap();
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
        self.g3m
            .init_pool(amounts.0.into(), amounts.1.into())
            .send()
            .await?
            .await?;
        Ok(())
    }
}
