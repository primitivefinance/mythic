use std::sync::Arc;

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use clients::protocol::{G3mF64, PoolInitParamsF64, ProtocolClient};
use ethers::{types::U256, utils::parse_ether};

use super::{agent::*, *};
use crate::agents::base::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct DcaG3mLiquidityProvider {
    pub client: Arc<ArbiterMiddleware>,
    pub protocol_client: ProtocolClient<ArbiterMiddleware>,
    pub token_x: Address,
    pub token_y: Address,
    initial_x_amount: f64,
    initial_price: f64,
    end_timestamp: f64,
}

#[async_trait::async_trait]
impl Agent for DcaG3mLiquidityProvider {
    #[tracing::instrument(skip(self), level = "trace")]
    async fn init(&mut self) -> Result<()> {
        let init_x = parse_ether(self.initial_x_amount)?;
        let init_price = parse_ether(self.initial_price)?;

        let init_params = PoolInitParamsF64::G3M(G3mF64 {
            wx: 0.02,
            swap_fee: 0.003,
            controller: self.client.address(),
        });

        let pool_id = self.protocol_client.get_next_pool_id().await?;

        self.protocol_client
            .init_pool(self.token_x, self.token_y, init_x, init_price, init_params)
            .await?;
        let set_weight_tx = self
            .protocol_client
            .set_weight_x(pool_id, 0.98, self.end_timestamp as u64)
            .await?;
        tracing::info!("tx: {:?}", set_weight_tx);
        tracing::info!("set weight on pool: {:?}", pool_id);

        Ok(())
    }

    fn client(&self) -> Arc<ArbiterMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DcaG3mLiquidityProvider {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        token_admin: &TokenAdmin,
        protocol_client: ProtocolClient<ArbiterMiddleware>,
    ) -> Result<Self> {
        let label = label.into();
        let client = ArbiterMiddleware::new(environment, Some(&label))?;
        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        let protocol_client = protocol_client.connect(client.clone())?;

        token_admin
            .mint(
                client.address(),
                parse_ether(10_000_000)?,
                parse_ether(10_000_000)?,
            )
            .await?;

        arbx.approve(protocol_client.protocol.address(), U256::MAX)
            .send()
            .await?;
        arby.approve(protocol_client.protocol.address(), U256::MAX)
            .send()
            .await?;

        if let Some(AgentParameters::DcaG3mLiquidityProvider(params)) =
            config.agent_parameters.get(&label).cloned()
        {
            Ok(Self {
                client,
                protocol_client,
                token_x: arbx.address(),
                token_y: arby.address(),
                initial_x_amount: params.initial_x_amount.0,
                initial_price: params.initial_price.0,
                end_timestamp: params.end_timestamp.0,
            })
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for `LiquidityProvider`"
            ))
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DcaG3mLiquidityProviderParameters<P: Parameterized> {
    pub initial_x_amount: P,
    pub initial_price: P,
    pub end_timestamp: P,
}

impl From<DcaG3mLiquidityProviderParameters<Multiple>>
    for Vec<DcaG3mLiquidityProviderParameters<Single>>
{
    fn from(params: DcaG3mLiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.initial_x_amount.parameters(),
            params.initial_price.parameters(),
            params.end_timestamp.parameters()
        )
        .map(|(ixa, ip, et)| DcaG3mLiquidityProviderParameters {
            initial_x_amount: Single(ixa),
            initial_price: Single(ip),
            end_timestamp: Single(et),
        })
        .collect()
    }
}
