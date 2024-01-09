use std::sync::Arc;

use alloy_primitives::{utils::parse_ether, Address, U256};
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use clients::protocol::ProtocolClient;
use tracing::debug;

use super::*;
use crate::agents::base::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct LiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    init_x_wad: f64,
    init_price_wad: f64,
    init_strike_price_wad: f64,
    init_sigma_percent_wad: f64,
    init_tau_years_wad: f64,
}

#[async_trait::async_trait]
impl Agent for LiquidityProvider {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn init(&mut self) -> Result<()> {
        debug!("LiquidityProvider initializing pool on DFMM.");
        self.protocol_client
            .initialize_pool(
                self.init_x_wad,
                self.init_price_wad,
                self.init_strike_price_wad,
                self.init_sigma_percent_wad,
                self.init_tau_years_wad,
            )
            .await?;

        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LiquidityProvider {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        token_admin: &TokenAdmin,
        market: Address,
        solver: Address,
    ) -> Result<Self> {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                from_ethers_address(client.address()),
                parse_ether("100")?,
                parse_ether("10_000_000")?,
            )
            .await?;

        arbx.approve(to_ethers_address(market), to_ethers_u256(U256::MAX))
            .send()
            .await?;
        arby.approve(to_ethers_address(market), to_ethers_u256(U256::MAX))
            .send()
            .await?;

        if let Some(AgentParameters::LiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            let protocol_client = ProtocolClient::new(
                client.clone(),
                to_ethers_address(market),
                to_ethers_address(solver),
            );

            // todo: right now we init pool with params, that can be updated obviously...

            Ok(Self {
                client,
                protocol_client,
                init_x_wad: params.x_liquidity.0,
                init_price_wad: params.initial_price.0,
                init_strike_price_wad: params.strike_price.0,
                init_sigma_percent_wad: params.sigma.0,
                init_tau_years_wad: params.tau.0,
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LiquidityProvider`"
            ))
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LiquidityProviderParameters<P: Parameterized> {
    pub x_liquidity: P,
    pub initial_price: P,
    pub sigma: P,
    pub tau: P,
    pub strike_price: P,
}

impl From<LiquidityProviderParameters<Multiple>> for Vec<LiquidityProviderParameters<Single>> {
    fn from(params: LiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.x_liquidity.parameters(),
            params.initial_price.parameters(),
            params.sigma.parameters(),
            params.tau.parameters(),
            params.strike_price.parameters()
        )
        .map(|(xl, ip, vol, tau, stk)| LiquidityProviderParameters {
            x_liquidity: Single(xl),
            initial_price: Single(ip),
            sigma: Single(vol),
            tau: Single(tau),
            strike_price: Single(stk),
        })
        .collect()
    }
}
