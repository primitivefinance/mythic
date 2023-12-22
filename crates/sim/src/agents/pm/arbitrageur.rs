use std::sync::Arc;

use alloy_primitives::{
    utils::{format_ether, format_units, parse_ether},
    Address, U256,
};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::errors::RevmMiddlewareError;
use clients::protocol::ProtocolClient;
use ethers::abi::AbiDecode;
use tracing::{debug, info, trace, warn};

use super::{
    agents::base::token_admin::TokenAdmin,
    bindings::atomic_v2::{self, AtomicV2},
    Environment, Result, RevmMiddleware, *,
};

pub const WAD: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

#[derive(Debug, Clone)]
pub struct Arbitrageur {
    pub client: Arc<RevmMiddleware>,
    /// Connects the Arbitrageur agent to the DFMM protocol.
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// Arbitrage vehicle for atomically swapping between exchanges.
    pub atomic_arbitrage: AtomicV2<RevmMiddleware>,
}

impl Arbitrageur {
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        dfmm_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Create the protocol client.
        let protocol_client = ProtocolClient::new(client.clone(), to_ethers_address(dfmm_address));

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange =
            LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        // Deploy the arbitrageur's atomic contract to atomically swap between
        // exchanges.
        let atomic_arbitrage = AtomicV2::deploy(
            client.clone(),
            (
                protocol_client.protocol.address(),
                liquid_exchange.address(),
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
                from_ethers_address(client.address()),
                parse_ether(100_000_000.to_string().as_str()).unwrap(),
                parse_ether(100_000_000.to_string().as_str()).unwrap(),
            )
            .await?;

        arbx.approve(atomic_arbitrage.address(), ethers::types::U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), ethers::types::U256::MAX)
            .send()
            .await?
            .await?;

        let arby_allowance = arby
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;
        let arbx_allowance = arbx
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;

        trace!("arbx_allowance: {:?}", arbx_allowance);
        trace!("arby_allowance: {:?}", arby_allowance);

        Ok(Self {
            client,
            protocol_client,
            liquid_exchange,
            atomic_arbitrage,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    #[tracing::instrument(skip(self), level = "trace", ret)]
    async fn detect_arbitrage(&self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let liquid_exchange_price_wad = from_ethers_u256(liquid_exchange_price_wad);

        let target_exchange_price_wad = self.protocol_client.get_internal_price().await?;
        let target_exchange_price_wad = from_ethers_u256(target_exchange_price_wad);
        info!("=== Start Loop ===");
        info!("Price[LEX]: {:?}", format_ether(liquid_exchange_price_wad));
        info!("Price[DEX]: {:?}", format_ether(target_exchange_price_wad));

        let swap_fee_wad = self.protocol_client.get_swap_fee().await?;
        let swap_fee_wad = from_ethers_u256(swap_fee_wad);
        let gamma_wad = WAD - swap_fee_wad;
        let upper_arb_bound = WAD * target_exchange_price_wad / gamma_wad;
        let lower_arb_bound = target_exchange_price_wad * gamma_wad / WAD;
        debug!("SwapFee [DEX]  : {:?}", format_ether(swap_fee_wad));
        debug!("ArbBound[UPPER]: {:?}", format_ether(upper_arb_bound));
        debug!("ArbBound[LOWER]: {:?}", format_ether(lower_arb_bound));

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound
            && liquid_exchange_price_wad > target_exchange_price_wad
        {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound
            && liquid_exchange_price_wad < target_exchange_price_wad
        {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_raise_price_trade_amount_y(&self, target_price_wad: U256) -> Result<U256> {
        let amount = self
            .atomic_arbitrage
            .try_arbitrage_until_target_price(to_ethers_u256(target_price_wad), 100.into())
            .call()
            .await;

        let amount = match amount {
            Ok(amount) => amount,
            Err(e) => {
                if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                    e.as_middleware_error().unwrap()
                {
                    warn!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);

                    // let decoded_output =
                    // atomic_v2::FindingTradeError::decode(&output)?;
                    // info!("Execution revert: {:?}", decoded_output);
                }

                return Ok(U256::ZERO);
            }
        };

        Ok(from_ethers_u256(amount))
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn get_lower_price_trade_amount_x(&self, target_price_wad: U256) -> Result<U256> {
        let amount = self
            .atomic_arbitrage
            .try_arbitrage_until_target_price(to_ethers_u256(target_price_wad), 100.into())
            .call()
            .await;

        let amount = match amount {
            Ok(amount) => amount,
            Err(e) => {
                if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                    e.as_middleware_error().unwrap()
                {
                    warn!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);

                    let decoded_output = atomic_v2::FindingTradeError::decode(output)?;
                    info!("Execution revert: {:?}", decoded_output);
                }

                return Ok(U256::ZERO);
            }
        };

        Ok(from_ethers_u256(amount))
    }

    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn raise_price_maximize_profit_search(&self) -> Result<U256> {
        let amount = self
            .atomic_arbitrage
            .search_raise_price(100.into(), 50.into())
            .call()
            .await;

        let amount = match amount {
            Ok((amount, _profit)) => amount,
            Err(e) => {
                if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                    e.as_middleware_error().unwrap()
                {
                    warn!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                }

                return Ok(U256::ZERO);
            }
        };

        Ok(from_ethers_u256(amount))
    }

    #[tracing::instrument(skip(self), level = "info", ret)]
    pub async fn lower_price_maximize_profit_search(&self) -> Result<U256> {
        self.atomic_arbitrage
            .search_lower_price(256.into(), 5.into())
            .send()
            .await?
            .await?;

        let trade_packet = self.atomic_arbitrage.trade_packet().call().await?;
        let amount = trade_packet.3;
        let profit = trade_packet.4;
        info!(
            "Trade: {:?} Profit: {:?}",
            format_ether(from_ethers_u256(amount)),
            format_ether(from_ethers_u256(profit))
        );

        // let amount = match amount {
        // Ok((amount, profit)) => {
        // tracing::info!(
        // "Trade: {:?} Profit: {:?}",
        // format_ether(from_ethers_u256(amount)),
        // format_ether(from_ethers_u256(profit))
        // );
        // amount
        // }
        // Err(e) => {
        // if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
        // e.as_middleware_error().unwrap()
        // {
        // warn!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
        // }
        //
        // return Ok(U256::ZERO);
        // }
        // };

        Ok(from_ethers_u256(amount))
    }

    pub async fn get_dx() -> Result<U256> {
        todo!()
    }

    pub async fn get_dy() -> Result<U256> {
        todo!()
    }
}
// TODO: make sure we're swapping on low and high vol strategies
#[async_trait::async_trait]
impl Agent for Arbitrageur {
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

        match self.detect_arbitrage().await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Signal[RAISE PRICE]: {:?}",
                    format_units(target_price, "ether")?
                );

                let input = self.raise_price_maximize_profit_search().await?;
                // let input = self.get_raise_price_trade_amount_y(target_price).await?;
                debug!("TradeInput[RAISE PRICE] {:?}", format_ether(input));

                let tx = self
                    .atomic_arbitrage
                    .raise_exchange_price(to_ethers_u256(input));
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
                            warn!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }

                let internal_price = self.protocol_client.get_internal_price().await?;
                let internal_price = from_ethers_u256(internal_price);
                info!("Price[LEX]: {:?}", format_ether(target_price));
                info!("Price[DEX]: {:?}", format_ether(internal_price));
                info!("=== End Loop ===");
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Signal[LOWER PRICE] {:?}",
                    format_units(target_price, "ether")?
                );

                let input = self.lower_price_maximize_profit_search().await?;
                // let input = self.get_lower_price_trade_amount_x(target_price).await?;

                debug!("TradeInput[LOWER PRICE] {:?}", format_ether(input));

                if input.is_zero() {
                    return Ok(());
                }

                let tx = self
                    .atomic_arbitrage
                    .lower_exchange_price(to_ethers_u256(input));
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
                            warn!("Execution revert: {:?}", output);
                        }
                    }
                }
                trace!("Sent arbitrage.");

                let internal_price = self.protocol_client.get_internal_price().await?;
                let internal_price = from_ethers_u256(internal_price);
                info!("Price[LEX]: {:?}", format_ether(target_price));
                info!("Price[DEX]: {:?}", format_ether(internal_price));
                info!("=== End Loop ===");
            }
            Swap::None => {
                trace!("No arbitrage opportunity");
            }
        }
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}

#[cfg(test)]
mod tests {}

pub fn to_float(value: U256) -> f64 {
    format_ether(value).parse::<f64>().unwrap()
}
