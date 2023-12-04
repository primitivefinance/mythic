use std::sync::Arc;

use alloy_primitives::{utils::parse_ether, Address, U256};
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;

use super::*;
use crate::agents::base::token_admin::TokenAdmin;

#[derive(Debug, Clone)]
pub struct LiquidityProvider {
    pub client: Arc<RevmMiddleware>,
    initial_x: U256,
    initial_price: U256,
}

#[async_trait::async_trait]
impl Agent for LiquidityProvider {
    async fn init(&mut self) -> Result<()> {
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
    ) -> Result<Self> {
        let label = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                from_ethers_address(client.address()),
                parse_ether("100")?,
                parse_ether("100")?,
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
            Ok(Self {
                client,
                initial_x: parse_ether(params.x_liquidity.0.to_string().as_str())?,
                initial_price: parse_ether(params.initial_price.0.to_string().as_str())?,
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
}

impl From<LiquidityProviderParameters<Multiple>> for Vec<LiquidityProviderParameters<Single>> {
    fn from(params: LiquidityProviderParameters<Multiple>) -> Self {
        itertools::iproduct!(
            params.x_liquidity.parameters(),
            params.initial_price.parameters()
        )
        .map(|(xl, ip)| LiquidityProviderParameters {
            x_liquidity: Single(xl),
            initial_price: Single(ip),
        })
        .collect()
    }
}
