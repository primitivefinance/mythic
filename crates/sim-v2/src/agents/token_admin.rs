use ethers::utils::parse_ether;

use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct TokenAdmin {
    pub init_token_data: Vec<TokenData>,
    #[serde(skip)]
    pub token_data: Option<HashMap<String, TokenData>>,
    #[serde(skip)]
    pub tokens: Option<HashMap<String, ArbiterToken<ArbiterMiddleware>>>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
}

impl TokenAdmin {
    async fn reply_token_data(&self, token_name: String, to: String) {
        let messager = self.messager.as_ref().unwrap();
        let token_data = self.token_data.as_ref().unwrap().get(&token_name).unwrap();
        messager.send(To::Agent(to), token_data).await;
    }
    async fn reply_address_of(&self, token_name: String, to: String) {
        let messager = self.messager.as_ref().unwrap();
        let token_data = self.token_data.as_ref().unwrap().get(&token_name).unwrap();
        messager.send(To::Agent(to), token_data.address).await;
    }

    async fn reply_get_asset_universe(&self, to: String) {
        let messager = self.messager.as_ref().unwrap();
        let asset_universe = self
            .token_data
            .as_ref()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<TokenData>>();

        messager.send(To::Agent(to), asset_universe).await;
    }

    async fn reply_mint_request(&self, mint_request: MintRequest, to: String) {
        let token = self
            .tokens
            .as_ref()
            .unwrap()
            .get(&mint_request.token)
            .unwrap();
        token
            .mint(
                mint_request.mint_to,
                parse_ether(mint_request.mint_amount).unwrap(),
            )
            .send()
            .await
            .unwrap();
    }
}

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    AddressOf(String),
    MintRequest(MintRequest),
    GetAssetUniverse,
    GetTokenData(String),
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<EventStream<Message>, ArbiterEngineError> {
        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        for token_data in &self.init_token_data {
            let mut token_data = token_data.clone();
            let token = ArbiterToken::deploy(
                client.clone(),
                (
                    token_data.name.clone(),
                    token_data.symbol.clone(),
                    token_data.decimals,
                ),
            )
            .unwrap()
            .send()
            .await
            .unwrap();

            token_data.address = Some(token.address());
            self.token_data
                .get_or_insert_with(HashMap::new)
                .insert(token_data.name.clone(), token_data.clone());
            self.tokens
                .get_or_insert_with(HashMap::new)
                .insert(token_data.name.clone(), token.clone());
        }
        Ok(messager.stream()?)
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow, ArbiterEngineError> {
        let query: TokenAdminQuery = serde_json::from_str(&event.data).unwrap();
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                self.reply_address_of(token_name, event.from).await;
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                self.reply_mint_request(mint_request, event.from).await;
            }
            TokenAdminQuery::GetAssetUniverse => {
                self.reply_get_asset_universe(event.from).await;
            }
            TokenAdminQuery::GetTokenData(token_name) => {
                self.reply_token_data(token_name, event.from).await
            }
        }
        Ok(Continue)
    }
}
