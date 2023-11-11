use std::sync::Arc;

use ethers::prelude::*;
use tracing::info;

const RPC_URL_WS: &str = "ws://localhost:8545";
const CHAIN_ID: u64 = 31337;

#[derive(Debug)]
pub struct Local<C> {
    pub client: Option<Arc<SignerMiddleware<Provider<C>, LocalWallet>>>,
}

impl Default for Local<Ws> {
    fn default() -> Self {
        Self { client: None }
    }
}

impl Local<Ws> {
    pub async fn new() -> anyhow::Result<Self> {
        // connect to the network
        let provider = Provider::<Ws>::connect(RPC_URL_WS).await?;
        info!("Connected to network at url {}", RPC_URL_WS);

        // Get private key from env variable
        let pk = std::env::var("PRIVATE_KEY_DEV").expect("PRIVATE_KEY env var not set");

        // make a wallet to use
        let wallet = pk.parse::<LocalWallet>()?.with_chain_id(CHAIN_ID);

        // connect the wallet to the provider
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self {
            client: Some(client.clone()),
        })
    }
}
