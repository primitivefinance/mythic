use std::sync::Arc;

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use clients::protocol::{LogNormalF64, PoolInitParamsF64, ProtocolClient};
use ethers::{types::U256, utils::parse_ether};

use super::{agent::*, *};
use crate::agents::base_agents::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct LogNormalLiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    pub protocol_client: ProtocolClient<RevmMiddleware>,
    pub controller: Address,
    pub token_x: Address,
    pub token_y: Address,
    initial_x_amount: f64,
    initial_price: f64,
    initial_strike: f64,
    initial_sigma: f64,
    initial_tau: f64,
}

#[async_trait::async_trait]
impl Agent for LogNormalLiquidityProvider {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn init(&mut self) -> Result<()> {
        let initial_x = parse_ether(self.initial_x_amount)?;
        let initial_price = parse_ether(self.initial_price)?;

        let next_pool_id = self.protocol_client.get_next_pool_id().await?;

        let init_params = PoolInitParamsF64::LogNormal(LogNormalF64 {
            strike: self.initial_strike,
            sigma: self.initial_sigma,
            tau: self.initial_tau,
            swap_fee: 0.003,
        });

        self.protocol_client
            .init_pool(
                self.token_x,
                self.token_y,
                initial_x,
                initial_price,
                init_params,
            )
            .await?;

        self.protocol_client
            .update_controller(next_pool_id, self.controller)
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

impl LogNormalLiquidityProvider {
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

        if let Some(AgentParameters::LogNormalLiquidityProvider(params)) =
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
                initial_strike: params.strike_price.0,
                initial_sigma: params.sigma.0,
                initial_tau: params.tau.0,
            })
        } else {
            anyhow::bail!("No parameters found for `LogNormalLiquidityProvider`")
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LogNormalLiquidityProviderParameters<P: Parameterized> {
    pub initial_x_amount: P,
    pub initial_price: P,
    pub strike_price: P,
    pub sigma: P,
    pub tau: P,
}

impl From<LogNormalLiquidityProviderParameters<Multiple>>
    for Vec<LogNormalLiquidityProviderParameters<Single>>
{
    fn from(params: LogNormalLiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.initial_x_amount.parameters(),
            params.initial_price.parameters(),
            params.strike_price.parameters(),
            params.sigma.parameters(),
            params.tau.parameters()
        )
        .map(
            |(ixa, ip, stk, sig, tau)| LogNormalLiquidityProviderParameters {
                initial_x_amount: Single(ixa),
                initial_price: Single(ip),
                strike_price: Single(stk),
                sigma: Single(sig),
                tau: Single(tau),
            },
        )
        .collect()
    }
}
