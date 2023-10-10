use std::ops::Div;

use super::*;

pub struct Arbitrageur {
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The geometric mean market.
    pub g3m: G3M<RevmMiddleware>,

    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,
}

pub enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}

impl Arbitrageur {
    pub async fn new(
        label: &str,
        environment: &Environment,
        liquid_exchange_address: Address,
        exchange_address: Address,
        // _exchange_parameters: OtherExchangeParameters,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, Some(label))?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let g3m = G3M::new(exchange_address, client.clone());
        let atomic_arbitrage =
            AtomicArbitrage::deploy(client, (exchange_address, liquid_exchange_address))?
                .send()
                .await?;

        Ok(Self {
            liquid_exchange,
            atomic_arbitrage,
            g3m,
        })
    }

    pub async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                let input = self.get_y_input(target_price).await?;
                self.atomic_arbitrage
                    .raise_exchange_price(input)
                    .send()
                    .await?
                    .await?;
            }
            Swap::LowerExchangePrice(target_price) => {
                let input = self.get_x_input(target_price).await?;
                self.atomic_arbitrage
                    .lower_exchange_price(input)
                    .send()
                    .await?
                    .await?;
            }
            Swap::None => {
                // info!("No arbitrage opportunity");
            }
        }
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
