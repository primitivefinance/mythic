use super::*;
use crate::math::*;

#[derive(Clone, Debug)]
pub struct MomentumStrategist {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub update_frequency: u64,
    pub next_update_timestamp: u64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_returns: Vec<(f64, u64)>,
    pub asset_returns: Vec<(f64, u64)>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MomentumParameters<P: Parameterized> {
    pub update_frequency: P,
}

impl From<MomentumParameters<Multiple>> for Vec<MomentumParameters<Single>> {
    fn from(item: MomentumParameters<Multiple>) -> Self {
        item.update_frequency
            .parameters()
            .into_iter()
            .map(|update_freq| MomentumParameters {
                update_frequency: Single(update_freq),
            })
            .collect()
    }
}

impl MomentumStrategist {
    fn calculate_returns(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.asset_prices.len() > 15 {
            let asset_return = compute_net_returns(
                self.asset_prices
                    .iter()
                    .skip(self.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.asset_returns
                .push((asset_return, self.next_update_timestamp));
        }
        if self.portfolio_prices.len() > 15 {
            let portfolio_return = compute_net_returns(
                self.portfolio_prices
                    .iter()
                    .skip(self.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.portfolio_returns
                .push((portfolio_return, self.next_update_timestamp));
        }
        trace!(
            "hypothetical percent asset return: {}",
            (self.asset_prices.last().unwrap().0 - self.asset_prices.first().unwrap().0)
                / self.asset_prices.first().unwrap().0
        );
        trace!(
            "portfolio percent return: {}",
            (self.portfolio_prices.last().unwrap().0 - self.portfolio_prices.first().unwrap().0)
                / self.portfolio_prices.first().unwrap().0
        );
        trace!(
            "initial portfolio price: {}",
            self.portfolio_prices.first().unwrap().0
        );
        trace!(
            "current portfolio price: {}",
            self.portfolio_prices.last().unwrap().0
        );

        Ok(())
    }
}

#[async_trait::async_trait]
impl G3mPortfolioManager for MomentumStrategist {
    // dumb poc, this just checks if the portfolio rv is greater than the target rv
    // then changes weight by 1% over the course of a day depending on if rv is
    // greater or less than target
    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.portfolio_returns.len() < 2 {
            return Ok(());
        }
        let portfolio_return = self.portfolio_returns.last().unwrap().0;
        info!("portfolio_rv: {}", portfolio_return);
        let current_weight_x = self.g3m.weight_x().call().await?;
        let current_weight_float = format_ether(current_weight_x).parse::<f64>().unwrap();
        info!("current_weight_float: {}", current_weight_float);
        if portfolio_return > 0.0 {
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

    fn g3m(&self) -> &G3M<RevmMiddleware> {
        &self.g3m
    }
}

#[async_trait::async_trait]
impl Agent for MomentumStrategist {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn step(&mut self) -> Result<()> {
        debug!("Entered `step()` for `MomentumStrategist`");
        if self.portfolio_prices.is_empty() {
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
            trace!("portfolio_price: {}", portfolio_price);

            self.portfolio_prices.push((portfolio_price, 0));
            self.asset_prices.push((asset_price, 0));
        }
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
            trace!("portfolio_price: {}", portfolio_price);

            self.asset_prices.push((asset_price, timestamp));
            self.portfolio_prices.push((portfolio_price, timestamp));

            self.calculate_returns()?;
            self.execute_smooth_rebalance().await?;
        }
        debug!("Finished `step()` for `MomentumStrategist`");
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
}
