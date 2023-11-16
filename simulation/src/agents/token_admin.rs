use std::{collections::BTreeMap, sync::Arc};

use arbiter_core::bindings::arbiter_token::ArbiterToken;
use ethers::abi::Tokenizable;

use super::*;

#[derive(Clone, Debug)]
pub struct TokenAdmin {
    pub client: Arc<RevmMiddleware>,
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenAdminParameters {
    arbx: TokenData,
    arby: TokenData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl TokenAdmin {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;

        if let Some(AgentParameters::TokenAdmin(parameters)) = config.agent_parameters.get(&label) {
            let arbx = ArbiterToken::deploy(
                client.clone(),
                (
                    parameters.arbx.name.clone(),
                    parameters.arbx.symbol.clone(),
                    parameters.arbx.decimals,
                ),
            )?
            .send()
            .await?;
            let arby = ArbiterToken::deploy(
                client.clone(),
                (
                    parameters.arby.name.clone(),
                    parameters.arby.symbol.clone(),
                    parameters.arby.decimals,
                ),
            )?
            .send()
            .await?;
            Ok(Self { client, arbx, arby })
        } else {
            Err(anyhow::anyhow!("No parameters found for token admin"))
        }
    }

    pub async fn mint(&self, to: Address, amount_x: U256, amount_y: U256) -> Result<()> {
        self.arbx.mint(to, amount_x).send().await?.await?;
        self.arby.mint(to, amount_y).send().await?.await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for TokenAdmin {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn startup(&mut self) -> Result<()> {
        Ok(())
    }

    async fn get_subscribed(&self) -> Result<Vec<SubscribedData>> {
        let total_x_supply = self.arbx.total_supply().call().await?;
        let total_y_supply = self.arby.total_supply().call().await?;

        let subbed = vec![
            SubscribedData::new("x_supply".to_string(), total_x_supply.into_token()),
            SubscribedData::new("y_supply".to_string(), total_y_supply.into_token()),
        ];

        Ok(subbed)
    }

    fn get_name(&self) -> String {
        "token_admin".to_string()
    }
}
