use super::*;
use crate::{agents::Agent, strategy::ArbitrageStrategy};

#[derive(Clone)]
pub struct RmmArbitrageur<S: ArbitrageStrategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// The low vol strategy used by the exchange.
    pub low_vol_strategy: S,
    /// The high vol strategy used by the exchange.
    pub high_vol_strategy: S,
    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,
    pub math: SD59x18Math<RevmMiddleware>,
}

impl<S: ArbitrageStrategy> RmmArbitrageur<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        liquid_exchange_address: Address,
        low_vol_strategy_address: Address,
        high_vol_strategy_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let low_vol_strategy = S::new(low_vol_strategy_address, client.clone());
        let high_vol_strategy = S::new(high_vol_strategy_address, client.clone());
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                low_vol_strategy_address,
                liquid_exchange_address,
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                high_vol_strategy_address,
                liquid_exchange_address,
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(100_000).unwrap(),
                parse_ether(100_000_000).unwrap(),
            )
            .await?;

        arbx.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;

        let math = SD59x18Math::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            low_vol_strategy,
            high_vol_strategy,
            math,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&self, pool: &S) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let price = pool.get_spot_price().await?;

        let gamma_wad = WAD - pool.get_swap_fee().await?;

        let upper_arb_bound = WAD * price / gamma_wad;
        let lower_arb_bound = price * gamma_wad / WAD;

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound && liquid_exchange_price_wad > price {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound && liquid_exchange_price_wad < price {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }
}
// TODO: make sure we're swapping on low and high vol strategies
#[async_trait::async_trait]
impl<S: ArbitrageStrategy + std::marker::Sync + std::marker::Send> Agent for RmmArbitrageur<S> {
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        for i in vec![&self.low_vol_strategy, &self.high_vol_strategy] {
            match self.detect_arbitrage(i).await? {
                Swap::RaiseExchangePrice(target_price) => {
                    info!(
                        "Detected the need to increase price to {:?}",
                        format_units(target_price, "ether")?
                    );
                    let input = self
                        .high_vol_strategy
                        .get_y_input(target_price, &self.math)
                        .await?;

                    info!(
                        "Increasing price by selling input amount of quote tokens: {:?}",
                        input,
                    );
                    let tx = self.atomic_arbitrage.raise_exchange_price(input);
                    let output = tx.send().await;
                    match output {
                        Ok(output) => {
                            output.await?;
                        }
                        Err(e) => {
                            if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                                e.as_middleware_error().unwrap()
                            {
                                info!("Execution revert: {:?}", output);
                            }
                        }
                    }
                }
                Swap::LowerExchangePrice(target_price) => {
                    info!(
                        "Detected the need to lower price to {:?}",
                        format_units(target_price, "ether")?
                    );
                    let input = self
                        .high_vol_strategy
                        .get_x_input(target_price, &self.math)
                        .await?;
                    info!("Got input: {:?}", input);
                    let tx = self.atomic_arbitrage.lower_exchange_price(input);
                    let output = tx.send().await;
                    match output {
                        Ok(output) => {
                            output.await?;
                        }
                        Err(e) => {
                            if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                                e.as_middleware_error().unwrap()
                            {
                                info!("Execution revert: {:?}", output);
                            }
                        }
                    }
                    info!("Sent arbitrage.");
                }
                Swap::None => {
                    info!("No arbitrage opportunity");
                }
            }
        }
        Ok(())
    }
}

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}
