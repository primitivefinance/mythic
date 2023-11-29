use super::*;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct RmmVolatilityTargetingStrategist {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub rmm: RMM<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct RmmVolatilityTargetingParameters<P: Parameterized> {
    pub target_volatility: P,
    pub update_frequency: P,
}

impl From<RmmVolatilityTargetingParameters<Multiple>>
    for Vec<RmmVolatilityTargetingParameters<Single>>
{
    fn from(item: RmmVolatilityTargetingParameters<Multiple>) -> Self {
        iproduct!(
            item.target_volatility.parameters(),
            item.update_frequency.parameters()
        )
        .map(|(tv, uf)| RmmVolatilityTargetingParameters {
            target_volatility: Single(tv),
            update_frequency: Single(uf),
        })
        .collect()
    }
}

impl RmmVolatilityTargetingStrategist {
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
impl RmmPortfolioManager for RmmVolatilityTargetingStrategist {
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.portfolio_rv.len() < 2 {
            return Ok(());
        }
        let portfolio_rv = self.portfolio_rv.last().unwrap().0;
        info!("portfolio_rv: {}", portfolio_rv);
        let rv_difference = portfolio_rv - self.target_volatility;
        let current_strike = self.rmm.strike_price().call().await?;
        let current_strike_float = format_ether(current_strike).parse::<f64>().unwrap();
        // let strike_change = self.sensitivity * rv_difference;
        info!("current strike float: {}", current_strike_float);
        let mut new_strike = current_strike_float;
        if portfolio_rv > self.target_volatility {
            new_strike -= 0.0015;
        } else {
            new_strike += 0.0015;
        }
        info!("new strike float: {}", new_strike);
        self.rmm
            .set_strike_price(
                parse_ether(new_strike.to_string()).unwrap(),
                U256::from(self.next_update_timestamp),
            )
            .send()
            .await?;
        Ok(())
    }
    fn rmm(&self) -> &RMM<RevmMiddleware> {
        &self.rmm
    }
}

#[async_trait::async_trait]
impl Agent for RmmVolatilityTargetingStrategist {
    async fn step(&mut self) -> Result<()> {
        let timestamp = self.client.get_block_timestamp().await?.as_u64();
        let asset_price = format_ether(self.lex.price().call().await?).parse::<f64>()?;
        let reserve_x = format_ether(self.rmm.reserve_x().call().await?).parse::<f64>()?;
        let reserve_y = format_ether(self.rmm.reserve_y().call().await?).parse::<f64>()?;
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

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
