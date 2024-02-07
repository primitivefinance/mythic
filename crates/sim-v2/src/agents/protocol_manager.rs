use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ProtocolManager {
    #[serde(skip)]
    protocol_client: Option<ProtocolClient<RevmMiddleware>>,
    #[serde(skip)]
    pub client: Option<Arc<RevmMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
}

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProtocolManagerQuery {
    Connect,
}

impl ProtocolManager {
    async fn reply_connect(&self, to: String) {
        let messager = self.messager.as_ref().unwrap();
        let addresses = self.protocol_client.as_ref().unwrap().addresses();
        messager.send(To::Agent(to), addresses).await;
    }
}

#[async_trait::async_trait]
impl Behavior<Message> for ProtocolManager {
    async fn startup(
        &mut self,
        client: Arc<RevmMiddleware>,
        messager: Messager,
    ) -> EventStream<Message> {
        let protocol_client = ProtocolClient::new(client.clone()).await.unwrap();
        println!("built protocol client!");
        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        self.protocol_client = Some(protocol_client);
        Box::pin(messager.stream())
    }

    async fn process(&mut self, event: Message) -> Option<MachineHalt> {
        let query: ProtocolManagerQuery = serde_json::from_str(&event.data).unwrap();
        match query {
            ProtocolManagerQuery::Connect => self.reply_connect(event.from).await,
        };
        None
    }
}
