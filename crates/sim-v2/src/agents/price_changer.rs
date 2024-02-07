use bindings::lex::Lex;

use self::token_admin::TokenAdminQuery;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct PriceChanger {
    #[serde(skip)]
    pub client: Option<Arc<RevmMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeployLexRequest {
    pub initial_price: f64,
    pub token_x: TokenData,
    pub amount_x: u64,
    pub token_y: TokenData,
    pub amount_y: u64,
}

impl PriceChanger {
    async fn reply_deploy_lex(&mut self, lex_params: DeployLexRequest, to: String) {
        let messager = self.messager.as_mut().unwrap();
        let lex = Lex::deploy(
            self.client.as_ref().unwrap().clone(),
            (
                lex_params.token_x.address.unwrap(),
                lex_params.token_y.address.unwrap(),
                parse_ether(lex_params.initial_price).unwrap(),
            ),
        )
        .unwrap()
        .send()
        .await
        .unwrap();

        let mint_requests = vec![
            MintRequest {
                token: lex_params.token_x.name.clone(),
                mint_to: lex.address(),
                mint_amount: lex_params.amount_x,
            },
            MintRequest {
                token: lex_params.token_y.name.clone(),
                mint_to: lex.address(),
                mint_amount: lex_params.amount_y,
            },
        ];

        for mint_request in mint_requests {
            messager
                .send(
                    To::Agent("token_admin".to_string()),
                    TokenAdminQuery::MintRequest(mint_request),
                )
                .await;
        }
        println!("deployed liquid exchange at: {:?}", lex.address());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PriceChangerQuery {
    DeployLexWithParams(DeployLexRequest),
}

#[async_trait::async_trait]
impl Behavior<Message> for PriceChanger {
    async fn startup(
        &mut self,
        client: Arc<RevmMiddleware>,
        messager: Messager,
    ) -> EventStream<Message> {
        self.client = Some(client.clone());
        self.messager = Some(messager.clone());
        Box::pin(messager.stream())
    }

    async fn process(&mut self, event: Message) -> Option<MachineHalt> {
        let query: PriceChangerQuery = serde_json::from_str(&event.data).unwrap();
        match query {
            PriceChangerQuery::DeployLexWithParams(lex_params) => {
                self.reply_deploy_lex(lex_params, event.from).await
            }
        };
        None
    }
}
