use std::sync::Arc;

use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use clients::protocol::ProtocolClient;
use ethers::{types::U256, utils::parse_ether};

use super::{
    agents::base_agents::token_admin::TokenAdmin,
    bindings::{arb_math::ArbMath, atomic_v2::AtomicV2},
    Environment, Result, RevmMiddleware, *,
};

#[derive(Debug, Clone)]
pub struct Arbitrageur {
    pub client: Arc<RevmMiddleware>,
    /// Connects the Arbitrageur agent to the DFMM protocol.
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// Arbitrage vehicle for atomically swapping between exchanges.
    pub atomic_arbitrage: AtomicV2<RevmMiddleware>,
    /// Arbiter math to do math for math utilities
    pub arb_math: ArbMath<RevmMiddleware>,
    /// Pool arbitrageur is responsible for trading against
    // todo(matt): refactor to handle n pools
    pub pool_id: U256,
    pub token_x: ArbiterToken<RevmMiddleware>,
    pub token_y: ArbiterToken<RevmMiddleware>,
}

impl Arbitrageur {
    pub async fn new(
        name: &str,
        environment: &Environment,
        token_admin: &TokenAdmin,
        liquid_exchange_address: Address,
        protocol_client: ProtocolClient<RevmMiddleware>,
        pool_id: U256,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, Some(name))?;
        let protocol_client = protocol_client.bind(client.clone())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());

        // Deploy the arbitrageur's atomic contract to atomically swap between
        // exchanges.
        let atomic_arbitrage = AtomicV2::deploy(
            client.clone(),
            (
                protocol_client.g_solver.address(),
                protocol_client.protocol.address(),
                liquid_exchange.address(),
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;

        let arb_math = ArbMath::deploy(client.clone(), ())?.send().await?;

        let token_x = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let token_y = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(100_000_000).unwrap(),
                parse_ether(100_000_000).unwrap(),
            )
            .await?;

        token_x
            .approve(atomic_arbitrage.address(), MAX)
            .send()
            .await?;
        token_y
            .approve(atomic_arbitrage.address(), MAX)
            .send()
            .await?;

        Ok(Self {
            client,
            protocol_client,
            liquid_exchange,
            atomic_arbitrage,
            arb_math,
            pool_id,
            token_x,
            token_y,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    #[tracing::instrument(skip(self), level = "trace", ret)]
    pub async fn detect_arbitrage(&self) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;

        let target_exchange_price_wad = self
            .protocol_client
            .get_internal_price(self.pool_id)
            .await?;

        match liquid_exchange_price_wad {
            _ if liquid_exchange_price_wad > target_exchange_price_wad => {
                Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
            }
            _ if liquid_exchange_price_wad < target_exchange_price_wad => {
                Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
            }
            _ => Ok(Swap::None),
        }
    }
}
