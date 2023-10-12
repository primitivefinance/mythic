use super::{token_admin::TokenAdmin, *};

pub const INITIAL_BALANCE: (u64, u64) = (100_000, 100_000);

#[derive(Clone)]
pub struct LiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
}

impl LiquidityProvider {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        g3m_address: Address,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "liquidity_provider".into())?;
        let g3m = G3M::new(g3m_address, client.clone());

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(INITIAL_BALANCE.0).unwrap(),
                parse_ether(INITIAL_BALANCE.1).unwrap(),
            )
            .await?;

        arbx.approve(g3m.address(), U256::MAX).send().await?;
        arby.approve(g3m.address(), U256::MAX).send().await?;

        Ok(Self { client, g3m })
    }

    pub async fn add_liquidity(self, config: &SimulationConfig) -> Result<()> {
        // Initial weight is set in the simulation config, but it can be overridden with
        // setWeightX() function.
        let initial_weight_0 =
            parse_ether(config.portfolio_pool_parameters.weight_token_0).unwrap();
        let initial_weight_1 = parse_ether(1)
            .unwrap()
            .checked_sub(initial_weight_0)
            .unwrap();
        // Using the initial weight, initial price, and initial reserve x, we can
        // compute reserve y.
        let initial_price = config.portfolio_pool_parameters.initial_price;
        let initial_reserve_x = parse_ether(INITIAL_BALANCE.0).unwrap();
        info!("initial_reserve_x: {}", initial_reserve_x);

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
        // Needs an amount of both tokens, the amounts can be anything but note that
        // they affect the spot price. TODO: The division by WAD here should be
        // removed once the contract is fixed.
        self.g3m
            .init_pool(amounts.0, amounts.1)
            .send()
            .await?
            .await?;
        Ok(())
    }
}
