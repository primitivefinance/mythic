use arbiter_core::bindings::arbiter_math::ArbiterMath;

use super::*;
use crate::{agents::Agent, strategy::ArbitrageStrategy};

#[derive(Clone)]
pub struct RmmArbitrageur<S: ArbitrageStrategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// The rmm strategy used by the exchange.
    pub rmm_strategy: S,
    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,
    pub g3m_math: SD59x18Math<RevmMiddleware>,
    pub rmm_math: ArbiterMath<RevmMiddleware>,
}

impl<S: ArbitrageStrategy> RmmArbitrageur<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        liquid_exchange_address: Address,
        rmm_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let rmm_strategy = S::new(rmm_address, client.clone());
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                rmm_address,
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

        let g3m_math = SD59x18Math::deploy(client.clone(), ())?.send().await?;
        let rmm_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            rmm_strategy,
            g3m_math,
            rmm_math,
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
        trace!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        trace!("rmm_price_wad: {:?}", price);

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
        let arbx = self.atomic_arbitrage.asset().call().await?;
        let arby = self.atomic_arbitrage.quote().call().await?;
        let arbx = ArbiterToken::new(arbx, self.client.clone());
        let arby = ArbiterToken::new(arby, self.client.clone());
        let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
        let arby_balance = arby.balance_of(self.client.address()).call().await?;
        trace!("arbx_balance: {:?}", arbx_balance);
        trace!("arby_balance: {:?}", arby_balance);

        match self.detect_arbitrage(&self.rmm_strategy).await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Detected the need to increase price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = self
                    .rmm_strategy
                    .get_y_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;

                info!(
                    "Increasing price by selling input amount of quote tokens: {:?}",
                    input,
                );
                if input < 0.into() {
                    return Ok(());
                }
                let tx = self.atomic_arbitrage.raise_exchange_price(input);
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                trace!("arbx_balance: {:?}", arbx_balance);
                trace!("arby_balance: {:?}", arby_balance);
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
                    .rmm_strategy
                    .get_x_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;
                info!("Got input: {:?}", input);
                if input <= 0.into() {
                    return Ok(());
                }
                let tx = self.atomic_arbitrage.lower_exchange_price(input);
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                trace!("arbx_balance after: {:?}", arbx_balance);
                trace!("arby_balance after: {:?}", arby_balance);
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
        Ok(())
    }
}

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}
