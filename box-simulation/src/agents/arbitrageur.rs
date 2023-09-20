use std::ops::Div;

use super::{atomic_arbitrage::AtomicArbitrage, *};

pub struct Prices {
    liquid_exchange_price_wad: U256,
    exchange_price_wad: U256,
}

pub struct Arbitrageur {
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    // pub exchange: Exchange<RevmMiddleware>,
    // pub exchange_parameters: ExchangeParameters,
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
        label: impl Into<String>,
        environment: &Environment,
        liquid_exchange_address: Address,
        exchange_address: Address,
        // _exchange_parameters: OtherExchangeParameters,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = Arc::new(RevmMiddleware::new(environment, Some(label.into()))?);

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        // let exchange = OtherExchange::new(exchange_address, client);
        let atomic_arbitrage =
            AtomicArbitrage::deploy(client, (exchange_address, liquid_exchange_address))?
                .send()
                .await?;

        Ok(Self {
            liquid_exchange,
            atomic_arbitrage,
            // exchange,
            // exchange_parameters: OtherExchangeParameters::new(),
        })
    }

    pub async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        Ok(match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                let input = self.get_quote_input(target_price).await?;
                self.atomic_arbitrage.raise_exchange_price(input);
            }
            Swap::LowerExchangePrice(target_price) => {
                let input = self.get_asset_input(target_price).await?;
                self.atomic_arbitrage.lower_exchange_price(input);
            }
            Swap::None => {
                // info!("No arbitrage opportunity");
            }
        })
    }

    async fn fetch_prices(&mut self) -> Result<Prices> {
        // TODO: Get the right price from the other exchange.
        Ok(Prices {
            liquid_exchange_price_wad: self.liquid_exchange.price().call().await?,
            exchange_price_wad: U256::from(0),
            // self.prices.exchange_price_wad = self.exchange.price().call().await?;
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&mut self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let Prices {
            liquid_exchange_price_wad,
            exchange_price_wad,
        } = self.fetch_prices().await?;

        // TODO: Get the right gamma from ExchangeParameters.
        let gamma_wad = U256::from(0);

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * exchange_price_wad / gamma_wad;
        let lower_arb_bound = exchange_price_wad * gamma_wad / WAD;

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound
            && liquid_exchange_price_wad > liquid_exchange_price_wad
        {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound
            && liquid_exchange_price_wad < exchange_price_wad
        {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }

    async fn get_quote_input(&mut self, target_price_wad: U256) -> Result<U256> {
        let (asset_weight, quote_weight) = self.exchange_parameters;
        let (asset_reserves, _quote_reserves, invariant) = self.exchange.status().call().await?;
        asset_weight
            * target_price_wad
                .div(invariant.pow(U256::from(1).div(quote_weight)))
                .pow(1 + asset_weight.div(quote_weight))
            - asset_reserves;
    }

    async fn get_asset_input(&mut self, target_price_wad: U256) -> Result<U256> {
        let (asset_weight, quote_weight) = self.exchange_parameters;
        let (_asset_reserves, quote_reserves, invariant) = self.exchange.status().call().await?;
        quote_weight
            * U256::from(1)
                .div(target_price_wad * invariant.pow(U256::from(1).div(asset_weight)))
                .pow(1 + quote_weight.div(asset_weight))
            - quote_reserves;
    }
}
