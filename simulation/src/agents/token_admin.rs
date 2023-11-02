use std::{collections::BTreeMap, sync::Arc};

use arbiter_core::bindings::arbiter_token::ArbiterToken;

use super::*;

#[derive(Clone)]
pub struct TokenAdmin {
    pub client: Arc<RevmMiddleware>,
    pub tokens: BTreeMap<String, ArbiterToken<RevmMiddleware>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenAdminParameters {
    tokens: Vec<TokenData>,
}

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
        let client = RevmMiddleware::new(environment, label.into())?;

        let mut tokens = vec![];
        if let AgentParameters::TokenAdmin(parameters) = config.agent_parameters.get(label.into()) {
            let token_list = parameters.tokens;
            for token in token_list {
                let token = ArbiterToken::deploy(
                    client.clone(),
                    (
                        token.name.clone(),
                        token.symbol.clone(),
                        token.decimals,
                        U256::MAX,
                    ),
                )?
                .send()
                .await?;
            }
        } else {
            return Err(anyhow::anyhow!("No parameters found for token admin"));
        }

        Ok(Self { client, tokens })
    }

    pub async fn mint(&self, to: Address, amount_x: U256, amount_y: U256) -> Result<()> {
        self.arbx.mint(to, amount_x).send().await?.await?;
        self.arby.mint(to, amount_y).send().await?.await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Agent for TokenAdmin {
    async fn startup(&mut self) -> Result<()> {
        Ok(())
    }
}
