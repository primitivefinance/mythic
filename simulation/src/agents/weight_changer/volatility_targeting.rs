use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct VolatilityTargetingStrategist {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub target_volatility: f64,
    pub update_frequency: u64,
    pub next_update_timestamp: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub sensitivity: f64,
    pub max_weight_change: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct VolatilityTargetingParameters<P: Parameterized> {
    pub target_volatility: P,
    pub update_frequency: P,
    pub initial_weight_x: P,
    pub fee_basis_points: P,
}

impl Into<Vec<VolatilityTargetingParameters<Single>>> for VolatilityTargetingParameters<Multiple> {
    fn into(self) -> Vec<VolatilityTargetingParameters<Single>> {
        self.target_volatility
            .parameters()
            .into_iter()
            .zip(self.update_frequency.parameters().into_iter())
            .zip(self.initial_weight_x.parameters().into_iter())
            .zip(self.fee_basis_points.parameters().into_iter())
            .map(|(((tv, uf), iwx), fbps)| VolatilityTargetingParameters {
                target_volatility: Single(tv),
                update_frequency: Single(uf),
                initial_weight_x: Single(iwx),
                fee_basis_points: Single(fbps),
            })
            .collect()
    }
}

impl VolatilityTargetingStrategist {
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
        debug!(
            "hypothetical percent asset return: {}",
            (self.asset_prices.last().unwrap().0 - self.asset_prices.first().unwrap().0)
                / self.asset_prices.first().unwrap().0
        );
        debug!(
            "portfolio percent return: {}",
            (self.portfolio_prices.last().unwrap().0 - self.portfolio_prices.first().unwrap().0)
                / self.portfolio_prices.first().unwrap().0
        );
        debug!(
            "initial portfolio price: {}",
            self.portfolio_prices.first().unwrap().0
        );
        debug!(
            "current portfolio price: {}",
            self.portfolio_prices.last().unwrap().0
        );

        Ok(())
    }
}

#[async_trait::async_trait]
impl WeightChanger for VolatilityTargetingStrategist {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.portfolio_rv.len() < 2 {
            return Ok(());
        }
        let portfolio_rv = self.portfolio_rv.last().unwrap().0;
        debug!("portfolio_rv: {}", portfolio_rv);
        let rv_difference = portfolio_rv - self.target_volatility;
        let current_weight_x = self.g3m.weight_x().call().await?;
        let current_weight_float = format_ether(current_weight_x).parse::<f64>().unwrap();
        let weight_change = self.sensitivity * rv_difference;
        debug!("current_weight_float: {}", current_weight_float);
        let mut weight_delta = weight_change;
        let mut new_weight = current_weight_float;
        if portfolio_rv < self.target_volatility {
            weight_delta = weight_change.min(self.max_weight_change);
            new_weight -= weight_delta;
            new_weight = new_weight.min(0.98);
        } else {
            weight_delta = (weight_change).min(self.max_weight_change);
            new_weight -= weight_delta;
            new_weight = new_weight.max(0.02);
        }
        debug!("new weight: {}", new_weight);
        debug!("weight delta: {}", weight_delta);
        self.g3m
            .set_weight_x(
                parse_ether(new_weight.to_string()).unwrap(),
                U256::from(self.next_update_timestamp),
            )
            .send()
            .await?;
        Ok(())
    }

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        &self.g3m
    }

    fn lex(&self) -> &LiquidExchange<RevmMiddleware> {
        &self.lex
    }
}

#[async_trait::async_trait]
impl Agent for VolatilityTargetingStrategist {
    async fn step(&mut self) -> Result<()> {
        let timestamp = self.client.get_block_timestamp().await?.as_u64();
        let asset_price = format_ether(self.lex.price().call().await?).parse::<f64>()?;
        let reserve_x =
            format_ether(self.g3m.reserve_x_without_precision().call().await?).parse::<f64>()?;
        let reserve_y =
            format_ether(self.g3m.reserve_y_without_precision().call().await?).parse::<f64>()?;
        let portfolio_price = reserve_x * asset_price + reserve_y;

        if self.portfolio_prices.is_empty() {
            info!("portfolio_price: {}", portfolio_price);
            self.portfolio_prices.push((portfolio_price, 0));
            self.asset_prices.push((asset_price, 0));
        }

        if timestamp >= self.next_update_timestamp {
            self.next_update_timestamp = timestamp + self.update_frequency;
            debug!("portfolio_price: {}", portfolio_price);
            self.asset_prices.push((asset_price, timestamp));
            self.portfolio_prices.push((portfolio_price, timestamp));
            self.calculate_rv()?;
            self.execute_smooth_rebalance().await?;
        }
        Ok(())
    }

    async fn startup(&mut self) -> Result<()> {
        Ok(())
    }
}
