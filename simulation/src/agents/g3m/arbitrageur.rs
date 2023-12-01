use ethers::abi::Tokenizable;

use super::*;
use crate::{agents::Agent, strategy::ArbitrageStrategy};

#[derive(Clone, Debug)]
pub struct Arbitrageur<S: ArbitrageStrategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The strategy used by the exchange.
    pub strategy: S,

    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,

    pub g3m_math: SD59x18Math<RevmMiddleware>,

    pub rmm_math: ArbiterMath<RevmMiddleware>,
}

impl<S: ArbitrageStrategy> Arbitrageur<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        liquid_exchange_address: Address,
        strategy_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let strategy = S::new(strategy_address, client.clone());
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                strategy_address,
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
                parse_ether(10_000).unwrap(),
                parse_ether(10_000).unwrap(),
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

        let g3m_math = SD59x18Math::deploy(client.clone(), ())?.send().await?;
        let rmm_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            strategy,
            g3m_math,
            rmm_math,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        trace!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        let g3m_price_wad = self.strategy.get_spot_price().await?;
        trace!("g3m_price_wad: {:?}", g3m_price_wad);

        trace!("swap_fee(): {:?}", self.strategy.get_swap_fee().await?);
        let gamma_wad = WAD - self.strategy.get_swap_fee().await?;
        trace!("gamma_wad: {:?}", gamma_wad);

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * g3m_price_wad / gamma_wad;
        trace!("upper_arb_bound: {:?}", upper_arb_bound);
        let lower_arb_bound = g3m_price_wad * gamma_wad / WAD;
        trace!("lower_arb_bound: {:?}", lower_arb_bound);

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
}

#[async_trait::async_trait]
impl<S: ArbitrageStrategy + std::marker::Sync + std::marker::Send + 'static> Agent
    for Arbitrageur<S>
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        debug!("Entered `step()` for arbitrageur");
        // Detect if there is an arbitrage opportunity.
        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                trace!(
                    "Detected the need to increase price to {:?}",
                    format_units(target_price, "ether")?
                );
                let (input, _output, _next_liquidity) = self
                    .strategy
                    .get_y_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;

                trace!(
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
                            trace!("Execution revert: {:?}", output);
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                trace!(
                    "Detected the need to lower price to {:?}",
                    format_units(target_price, "ether")?
                );
                let (input, _output, _next_liquidity) = self
                    .strategy
                    .get_x_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;
                trace!("Got input: {:?}", input);
                let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
                let input = input * liquid_exchange_price_wad / WAD + 1;
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
                            debug!("Execution revert: {:?}", output);
                            let NotProfitable {
                                first_swap_output,
                                second_swap_output,
                            } = NotProfitable::decode(output)?;
                            trace!(
                                "first_swap_output: {:?}",
                                format_units(first_swap_output, "ether")?
                            );
                            trace!(
                                "second_swap_output: {:?}",
                                format_units(second_swap_output, "ether")?
                            );
                        }
                    }
                }
                trace!("Sent arbitrage.");
            }
            Swap::None => {
                trace!("No arbitrage opportunity");
            }
        }
        debug!("Finished `step()` for `Arbitrageur`");
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    async fn get_subscribed(&self) -> Result<Vec<SubscribedData>> {
        let atomic = self.atomic_arbitrage.address();
        let liquid = self.liquid_exchange.address();
        let x_token = self.atomic_arbitrage.asset().call().await?;
        let y_token = self.atomic_arbitrage.quote().call().await?;
        let this = self.client.address();

        let x_balance = ArbiterToken::new(x_token, self.client.clone())
            .balance_of(this)
            .call()
            .await?;

        let y_balance = ArbiterToken::new(y_token, self.client.clone())
            .balance_of(this)
            .call()
            .await?;

        let subbed = vec![
            SubscribedData::new("Balance X".to_string(), x_balance.into_token()),
            SubscribedData::new("Balance Y".to_string(), y_balance.into_token()),
        ];

        Ok(subbed)
    }

    fn get_name(&self) -> String {
        "Arbitrageur".to_string()
    }
}

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}
