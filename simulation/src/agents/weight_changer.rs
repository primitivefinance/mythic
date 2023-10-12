use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct WeightChanger {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub target_volatility: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

impl WeightChanger {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig,
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
            target_volatility: config.pool.target_volatility,
            next_update_timestamp: 0,
            portfolio_prices: Vec::new(),
            asset_prices: Vec::new(),
            portfolio_rv: Vec::new(),
            asset_rv: Vec::new(),
        })
    }

    pub async fn step(&mut self, timestamp: u64) -> Result<()> {
        if timestamp > self.next_update_timestamp {
            self.next_update_timestamp = timestamp + 86400;
            let asset_price = self.lex.price().call().await?;

            let reserve_x = self.g3m.reserve_x().call().await?;
            let reserve_y = self.g3m.reserve_y().call().await?;
            let liquidity = self.g3m.total_liquidity().call().await?;
            let x_per_liquidity = reserve_x.div(liquidity);
            let y_per_liquidity = reserve_y.div(liquidity);

            let portfolio_price = x_per_liquidity
                .checked_mul(asset_price)
                .unwrap()
                .checked_add(y_per_liquidity)
                .unwrap();

            let asset_price_float = format_ether(asset_price).parse::<f64>().unwrap();
            let portfolio_price_float = format_ether(portfolio_price).parse::<f64>().unwrap();

            self.asset_prices.push((asset_price_float, timestamp));
            self.portfolio_prices
                .push((portfolio_price_float, timestamp));
            self.calculate_rv()?;
            self.execute_smooth_rebalance().await?;
        }
        Ok(())
    }

    fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calcualte for the last 15 elements
        if self.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.asset_prices
                    .iter()
                    .skip(self.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.asset_rv.push((asset_rv, self.next_update_timestamp));
        } else {
            let asset_rv = compute_realized_volatility(
                self.asset_prices
                    .iter()
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
        } else {
            let portfolio_rv = compute_realized_volatility(
                self.portfolio_prices
                    .iter()
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }

        Ok(())
    }

    // dumb poc, this just checks if the portfolio rv is greater than the target rv
    // then changes weight by 1% over the course of a day depending on if rv is
    // greater or less than target
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        let portfolio_rv = self.portfolio_rv.last().unwrap().0;
        let current_weight_x = self.g3m.weight_x().call().await?;
        let current_weight_float = format_ether(current_weight_x).parse::<f64>().unwrap();
        if portfolio_rv < self.target_volatility {
            let new_weight = current_weight_float + 0.01;
            self.g3m
                .set_weight_x(
                    parse_ether(new_weight.to_string()).unwrap(),
                    U256::from(86400),
                )
                .send()
                .await?;
        } else {
            let new_weight = current_weight_float - 0.01;
            self.g3m
                .set_weight_x(
                    parse_ether(new_weight.to_string()).unwrap(),
                    U256::from(86400),
                )
                .send()
                .await?;
        }
        Ok(())
    }
}
