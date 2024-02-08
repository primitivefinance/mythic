use clients::protocol::{LogNormalF64, PoolInitParamsF64, ProtocolClientAddresses};

use self::{
    price_changer::{DeployLexRequest, PriceChangerQuery},
    protocol_manager::ProtocolManagerQuery,
    token_admin::TokenAdminQuery,
};
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LiquidityProvider {
    pool_init_data: Vec<PoolInitData>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    #[serde(skip)]
    protocol_client: Option<ProtocolClient<ArbiterMiddleware>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolInitData {
    token_x: String,
    token_y: String,
    init_x_amount: f64,
    init_price: f64,
    strike: f64,
    sigma: f64,
    tau: f64,
}

#[async_trait::async_trait]
impl Behavior<Message> for LiquidityProvider {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        mut messager: Messager,
    ) -> Result<EventStream<Message>, ArbiterEngineError> {
        messager
            .send(
                To::Agent("protocol_manager".to_string()),
                ProtocolManagerQuery::Connect,
            )
            .await?;
        let protocol_addresses = messager.get_next().await?;
        let protocol_addresses =
            serde_json::from_str::<ProtocolClientAddresses>(&protocol_addresses.data).unwrap();
        let protocol_client =
            ProtocolClient::from_deployed(client.clone(), protocol_addresses).unwrap();

        for pool in &self.pool_init_data {
            let mint_requests = vec![
                MintRequest {
                    token: pool.token_x.clone(),
                    mint_to: client.address(),
                    mint_amount: 100,
                },
                MintRequest {
                    token: pool.token_y.clone(),
                    mint_to: client.address(),
                    mint_amount: 100,
                },
            ];

            for mint_request in mint_requests {
                messager
                    .send(
                        To::Agent("token_admin".to_string()),
                        TokenAdminQuery::MintRequest(mint_request),
                    )
                    .await?;
            }

            messager
                .send(
                    To::Agent("token_admin".to_string()),
                    TokenAdminQuery::GetTokenData(pool.token_x.clone()),
                )
                .await?;
            let token_x_data = messager.get_next().await?;
            let token_x_data = serde_json::from_str::<TokenData>(&token_x_data.data).unwrap();
            let token_x = ArbiterToken::new(token_x_data.address.unwrap(), client.clone());
            token_x
                .approve(protocol_client.protocol.address(), MAX)
                .send()
                .await
                .unwrap();

            messager
                .send(
                    To::Agent("token_admin".to_string()),
                    TokenAdminQuery::GetTokenData(pool.token_y.clone()),
                )
                .await?;
            let token_y_data = messager.get_next().await?;
            let token_y_data = serde_json::from_str::<TokenData>(&token_y_data.data)?;
            let token_y = ArbiterToken::new(token_y_data.address.unwrap(), client.clone());
            token_y
                .approve(protocol_client.protocol.address(), MAX)
                .send()
                .await
                .unwrap();

            let pool_init_params = PoolInitParamsF64::LogNormal(LogNormalF64 {
                strike: pool.strike,
                sigma: pool.sigma,
                tau: pool.tau,
                swap_fee: 0.003,
            });

            protocol_client
                .init_pool(
                    token_x.address(),
                    token_y.address(),
                    pool.init_x_amount,
                    pool.init_price,
                    pool_init_params,
                )
                .await
                .unwrap();
            println!("deployed pool!");
            let lex_request = DeployLexRequest {
                initial_price: pool.init_price,
                token_x: token_x_data,
                amount_x: 1_000_000,
                token_y: token_y_data,
                amount_y: 1_000_000,
            };

            messager
                .send(
                    To::Agent("price_changer".to_string()),
                    PriceChangerQuery::DeployLexWithParams(lex_request),
                )
                .await?;
            let reply = messager.get_next().await;
            println!("got reply from lex deployer: {:?}", reply);
        }

        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        self.protocol_client = Some(protocol_client);
        Ok(messager.stream()?)
    }

    async fn process(&mut self, _event: Message) -> Result<ControlFlow, ArbiterEngineError> {
        // Liquidity provider does nothing outside of `startup` stage
        Ok(Halt)
    }
}
