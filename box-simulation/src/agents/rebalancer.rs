use box_core::math::ComputeReturns;
use ethers::utils::format_ether;
use std::ops::Div;

use super::*;

pub struct Rebalancer {
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub target_volatility: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

impl Rebalancer {
    pub async fn new(
        label: &str,
        environment: &Environment,
        liquid_exchange_address: Address,
        exchange_address: Address,
        target_volatility: f64,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, Some(label))?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());
        let g3m = G3M::new(exchange_address, client.clone());

        Ok(Self {
            lex,
            g3m,
            target_volatility,
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
        }
        Ok(())
    }

    pub fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calcualte for the last 15 elements
        if self.asset_prices.len() > 15 {
            let asset_prices = self
                .asset_prices
                .iter()
                .skip(self.asset_prices.len() - 15)
                .map(|(price, _)| price.clone())
                .collect::<Vec<f64>>();

            let asset_rv = asset_prices.compute_realized_volatility();
            self.asset_rv.push((asset_rv, self.next_update_timestamp));
        } else {
            let asset_prices = self
                .asset_prices
                .iter()
                .map(|(price, _)| price.clone())
                .collect::<Vec<f64>>();
            let asset_rv = asset_prices.compute_realized_volatility();
            self.asset_rv.push((asset_rv, self.next_update_timestamp));
        }

        if self.portfolio_prices.len() > 15 {
            let portfolio_prices = self
                .portfolio_prices
                .iter()
                .skip(self.portfolio_prices.len() - 15)
                .map(|(price, _)| price.clone())
                .collect::<Vec<f64>>();

            let portfolio_rv = portfolio_prices.compute_realized_volatility();
            self.portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        } else {
            let portfolio_prices = self
                .portfolio_prices
                .iter()
                .map(|(price, _)| price.clone())
                .collect::<Vec<f64>>();
            let portfolio_rv = portfolio_prices.compute_realized_volatility();
            self.portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }

        Ok(())
    }
}
