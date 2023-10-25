use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct WeightChanger {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

impl WeightChanger {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Fixed>,
        liquid_exchange_address: Address,
        arbx: Address,
        arby: Address,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "weight_changer".into())?;

        let g3m_args = (
            arbx,
            arby,
            ethers::utils::parse_ether(config.pool.weight_x)?,
            U256::from(config.pool.fee_basis_points),
        );
        let g3m = G3M::deploy(client.clone(), g3m_args)?.send().await?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());
        Ok(Self {
            client,
            lex,
            g3m,
            target_volatility: config.weight_changer.target_volatility,
            update_frequency: config.weight_changer.update_frequency,
            next_update_timestamp: config.weight_changer.update_frequency,
            portfolio_prices: Vec::new(),
            asset_prices: Vec::new(),
            portfolio_rv: Vec::new(),
            asset_rv: Vec::new(),
        })
    }

    fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.asset_prices
                    .iter()
                    .skip(self.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.asset_rv.push((asset_rv, self.next_update_timestamp));
        }
        if self.portfolio_prices.len() > 15 {
            let portfolio_rv = compute_realized_volatility(
                self.portfolio_prices
                    .iter()
                    .skip(self.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }
        info!(
            "hypothetical percent asset return: {}",
            (self.asset_prices.last().unwrap().0 - self.asset_prices.first().unwrap().0)
                / self.asset_prices.first().unwrap().0
        );
        info!(
            "portfolio percent return: {}",
            (self.portfolio_prices.last().unwrap().0 - self.portfolio_prices.first().unwrap().0)
                / self.portfolio_prices.first().unwrap().0
        );
        info!(
            "initial portfolio price: {}",
            self.portfolio_prices.first().unwrap().0
        );
        info!(
            "current portfolio price: {}",
            self.portfolio_prices.last().unwrap().0
        );

        Ok(())
    }

    // dumb poc, this just checks if the portfolio rv is greater than the target rv
    // then changes weight by 1% over the course of a day depending on if rv is
    // greater or less than target
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.portfolio_rv.len() < 2 {
            return Ok(());
        }
        let portfolio_rv = self.portfolio_rv.last().unwrap().0;
        info!("portfolio_rv: {}", portfolio_rv);
        let current_weight_x = self.g3m.weight_x().call().await?;
        let current_weight_float = format_ether(current_weight_x).parse::<f64>().unwrap();
        info!("current_weight_float: {}", current_weight_float);
        if portfolio_rv < self.target_volatility {
            let new_weight = current_weight_float + 0.0025;
            info!("new weight: {}", new_weight);
            self.g3m
                .set_weight_x(
                    parse_ether(new_weight.to_string()).unwrap(),
                    U256::from(self.next_update_timestamp),
                )
                .send()
                .await?;
        } else {
            let new_weight = current_weight_float - 0.0025;
            info!("new weight: {}", new_weight);
            self.g3m
                .set_weight_x(
                    parse_ether(new_weight.to_string()).unwrap(),
                    U256::from(self.next_update_timestamp),
                )
                .send()
                .await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for WeightChanger {
    async fn step(&mut self) -> Result<()> {
        let timestamp = self.client.get_block_timestamp().await?.as_u64();
        if timestamp >= self.next_update_timestamp {
            self.next_update_timestamp = timestamp + self.update_frequency;
            let asset_price = format_ether(self.lex.price().call().await?)
                .parse::<f64>()
                .unwrap();

            let reserve_x = format_ether(self.g3m.reserve_x_without_precision().call().await?)
                .parse::<f64>()
                .unwrap();
            let reserve_y = format_ether(self.g3m.reserve_y_without_precision().call().await?)
                .parse::<f64>()
                .unwrap();

            let portfolio_price = reserve_x * asset_price + reserve_y;
            info!("portfolio_price: {}", portfolio_price);

            self.asset_prices.push((asset_price, timestamp));
            self.portfolio_prices.push((portfolio_price, timestamp));
            // info!("asset_prices: {:?}", self.asset_prices);
            // info!("portfolio_prices: {:?}", self.portfolio_prices);
            self.calculate_rv()?;
            self.execute_smooth_rebalance().await?;
        }
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        let asset_price = format_ether(self.lex.price().call().await?)
            .parse::<f64>()
            .unwrap();

        let reserve_x = format_ether(self.g3m.reserve_x_without_precision().call().await?)
            .parse::<f64>()
            .unwrap();
        let reserve_y = format_ether(self.g3m.reserve_y_without_precision().call().await?)
            .parse::<f64>()
            .unwrap();

        let portfolio_price = reserve_x * asset_price + reserve_y;
        info!("portfolio_price: {}", portfolio_price);

        self.portfolio_prices.push((portfolio_price, 0));
        self.asset_prices.push((asset_price, 0));

        Ok(())
    }
}
