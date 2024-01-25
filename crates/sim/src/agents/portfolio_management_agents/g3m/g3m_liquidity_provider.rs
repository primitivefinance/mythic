use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use clients::protocol::{G3mF64, PoolInitParamsF64, ProtocolClient};
use ethers::{types::U256, utils::parse_ether};
use std::sync::Arc;

use super::{agent::*, *};
use crate::agents::base_agents::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct G3mLiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    pub controller: Address,
    pub token_x: Address,
    pub token_y: Address,
    initial_x_amount: f64,
    initial_price: f64,
    initial_wx: f64,
}

#[async_trait::async_trait]
impl Agent for G3mLiquidityProvider {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn init(&mut self) -> Result<()> {
        let init_x = parse_ether(self.init_x_wad)?;
        let init_price = parse_ether(self.init_price_wad)?;

        let init_params = PoolInitParamsF64::G3M(G3mF64 {
            wx: self.init_weight_x_wad,
            swap_fee: 0.003,
        });

        let next_pool_id = self.protocol_client.next_pool_id().await?;

        self.protocol_client
            .init_pool(self.token_x, self.token_y, init_x, init_price, init_params)
            .await?;

        self.protocol_client
            .update_controller(ethers::types::U256::from(1), self.controller)
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

impl G3mLiquidityProvider {
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

        let controller = protocol_client.client.address();

        let protocol_client = protocol_client.bind(client.clone())?;

        token_admin
            .mint(
                client.address(),
                parse_ether("10_000_000")?,
                parse_ether("10_000_000")?,
            )
            .await?;

        arbx.approve(protocol_client.protocol.address(), U256::MAX)
            .send()
            .await?;
        arby.approve(protocol_client.protocol.address(), U256::MAX)
            .send()
            .await?;

        if let Some(AgentParameters::LiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            Ok(Self {
                client,
                protocol_client,
                controller,
                token_x: arbx.address(),
                token_y: arby.address(),
                initial_x_amount: params.initial_x_amount.0,
                initial_price: params.initial_price.0,
                initial_wx: params.wx.0,
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LiquidityProvider`"
            ))
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct G3mLiquidityProviderParameters<P: Parameterized> {
    pub initial_x_amount: P,
    pub initial_price: P,
    pub wx: P,
}

impl From<G3mLiquidityProviderParameters<Multiple>>
    for Vec<G3mLiquidityProviderParameters<Single>>
{
    fn from(params: G3mLiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.initial_x_amount.parameters(),
            params.initial_price.parameters(),
            params.wx.parameters()
        )
        .map(|(ixa, ip, w)| G3mLiquidityProviderParameters {
            initial_x_amount: Single(ixa),
            initial_price: Single(ip),
            wx: Single(w),
        })
        .collect()
    }
}
