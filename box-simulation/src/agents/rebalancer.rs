use box_core::math::ComputeReturns;
use ethers::utils::format_ether;
use std::collections::VecDeque;
use std::ops::Div;

use super::*;

pub struct Rebalancer {
    pub lex: LiquidExchange<RevmMiddleware>,
    pub g3m: G3M<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub target_volatility: f64,
    pub asset_vol: U256,
    pub portfolio_vol: U256,
    pub last_asset_value: f64,
    pub last_portfolio_value: f64,
    pub portfolio_prices: Vec<f64>,
    pub asset_prices: Vec<f64>,
    pub portfolio_returns: VecDeque<f64>,
    pub asset_returns: VecDeque<f64>,
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
            asset_vol: U256::from(0),
            portfolio_vol: U256::from(0),
            last_asset_value: 0.0,
            last_portfolio_value: 0.0,
            portfolio_prices: Vec::new(),
            asset_prices: Vec::new(),
            portfolio_returns: VecDeque::new(),
            asset_returns: VecDeque::new(),
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

            self.append_asset_return(asset_price_float)?;
            self.append_portfolio_return(portfolio_price_float)?;
        }
        Ok(())
    }

    pub fn append_asset_return(&mut self, asset_price: f64) -> Result<()> {
        if self.asset_returns.len() == 15 {
            self.asset_returns.pop_front();
        }
        if self.last_asset_value == 0.0 {
            self.asset_returns.push_back(0.0);
        } else {
        }
        self.asset_returns.push_back(new_asset_return);

        Ok(())
    }

    pub fn append_portfolio_return(&mut self, portfolio_price: f64) -> Result<()> {
        if self.portfolio_returns.len() == 15 {
            self.portfolio_returns.pop_front();
        }
        self.portfolio_returns.push_back(new_portfolio_return);

        Ok(())
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&mut self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let g3m_price_wad = self.g3m.get_spot_price().call().await?;

        let gamma_wad = WAD - self.g3m.swap_fee().call().await?;

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * g3m_price_wad / gamma_wad;
        let lower_arb_bound = g3m_price_wad * gamma_wad / WAD;

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound && liquid_exchange_price_wad > g3m_price_wad
        {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound
            && liquid_exchange_price_wad < g3m_price_wad
        {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }

    async fn get_x_input(&mut self, target_price_wad: U256) -> Result<U256> {
        let weight_x = self.g3m.weight_x().call().await?;
        let weight_y = self.g3m.weight_y().call().await?;
        let reserve_y = self.g3m.reserve_y().call().await?;
        let invariant = self.g3m.get_invariant().call().await?;

        Ok(weight_y
            * U256::from(1)
                .div(target_price_wad * invariant.pow(U256::from(1).div(weight_x)))
                .pow(U256::from(1) + weight_y.div(weight_x))
            - reserve_y)
    }

    async fn get_y_input(&mut self, target_price_wad: U256) -> Result<U256> {
        let weight_x = self.g3m.weight_x().call().await?;
        let weight_y = self.g3m.weight_y().call().await?;
        let reserve_x = self.g3m.reserve_x().call().await?;
        let invariant = self.g3m.get_invariant().call().await?;

        Ok(weight_x
            * target_price_wad
                .div(invariant.pow(U256::from(1).div(weight_y)))
                .pow(U256::from(1) + weight_x.div(weight_y))
            - reserve_x)
    }
}
