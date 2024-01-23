use std::sync::Arc;

use alloy_primitives::{utils::parse_ether, U256};
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use clients::protocol::{G3mF64, LogNormalF64, PoolInitParamsF64, ProtocolClient};

use super::*;
use crate::agents::base::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct LiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    token_x: Address,
    token_y: Address,
    init_x_wad: f64,
    init_price_wad: f64,
    init_strike_price_wad: f64,
    init_sigma_percent_wad: f64,
    init_tau_years_wad: f64,
    init_weight_x_wad: f64,
}

#[async_trait::async_trait]
impl Agent for LiquidityProvider {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn init(&mut self) -> Result<()> {
        let init_x = ethers::utils::parse_ether(self.init_x_wad)?;
        let init_price = ethers::utils::parse_ether(self.init_price_wad)?;

        // todo: it's a little dumb to be storing the init params on the liquidity
        // provider struct...
        let ln_init_params = PoolInitParamsF64::LogNormal(LogNormalF64 {
            strike: self.init_strike_price_wad,
            sigma: self.init_sigma_percent_wad,
            tau: self.init_tau_years_wad,
            swap_fee: 0.003,
        });

        self.protocol_client
            .init_pool(
                self.token_x,
                self.token_y,
                init_x,
                init_price,
                ln_init_params,
            )
            .await?;

        let g_init_params = PoolInitParamsF64::G3M(G3mF64 {
            wx: self.init_weight_x_wad,
            swap_fee: 0.003,
        });

        self.protocol_client
            .init_pool(
                self.token_x,
                self.token_y,
                init_x,
                init_price,
                g_init_params,
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
        protocol_client: ProtocolClient<RevmMiddleware>,
    ) -> Result<Self> {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        let protocol_client = protocol_client.bind(client.clone())?;

        token_admin
            .mint(
                from_ethers_address(client.address()),
                parse_ether("10_000_000")?,
                parse_ether("10_000_000")?,
            )
            .await?;

        arbx.approve(
            protocol_client.protocol.address(),
            to_ethers_u256(U256::MAX),
        )
        .send()
        .await?;
        arby.approve(
            protocol_client.protocol.address(),
            to_ethers_u256(U256::MAX),
        )
        .send()
        .await?;

        if let Some(AgentParameters::LiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            Ok(Self {
                client,
                protocol_client,
                token_x: arbx.address(),
                token_y: arby.address(),
                init_x_wad: params.x_liquidity.0,
                init_price_wad: params.initial_price.0,
                init_strike_price_wad: params.strike_price.0,
                init_sigma_percent_wad: params.sigma.0,
                init_tau_years_wad: params.tau.0,
                init_weight_x_wad: params.wx.0,
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
    pub wx: P,
}

impl From<LiquidityProviderParameters<Multiple>> for Vec<LiquidityProviderParameters<Single>> {
    fn from(params: LiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.x_liquidity.parameters(),
            params.initial_price.parameters(),
            params.sigma.parameters(),
            params.tau.parameters(),
            params.strike_price.parameters(),
            params.wx.parameters()
        )
        .map(|(xl, ip, vol, tau, stk, w)| LiquidityProviderParameters {
            x_liquidity: Single(xl),
            initial_price: Single(ip),
            sigma: Single(vol),
            tau: Single(tau),
            strike_price: Single(stk),
            wx: Single(w),
        })
        .collect()
    }
}
