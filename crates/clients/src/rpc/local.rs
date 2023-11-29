use std::sync::Arc;

use ethers::{
    prelude::*,
    utils::{Anvil, AnvilInstance},
};
use simulation::bindings::{coin::Coin, counter::Counter};
use tracing::info;

const RPC_URL_WS: &str = "ws://localhost:8545";
const CHAIN_ID: u64 = 31337;

#[derive(Clone)]
pub struct Local<C> {
    pub client: Option<Arc<SignerMiddleware<Provider<C>, LocalWallet>>>,
    pub anvil: Option<Arc<AnvilInstance>>,
    pub counter_contract: Option<Address>,
    pub coin_contract: Option<Address>,
}

impl<C> std::fmt::Debug for Local<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Local")
            .field("client", &self.client.is_some())
            .field("anvil", &self.anvil.is_some())
            .finish()
    }
}

impl Default for Local<Ws> {
    fn default() -> Self {
        Self {
            client: None,
            anvil: None,
            counter_contract: None,
            coin_contract: None,
        }
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
            anvil: None,
            counter_contract: None,
            coin_contract: None,
        })
    }

    /// For running anvil in the background of Excalibur.
    pub fn with_anvil(self) -> Self {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(CHAIN_ID)
            .spawn();

        Self {
            anvil: Some(Arc::new(anvil)),
            ..self
        }
    }

    /// For using the default anvil wallet.
    pub async fn with_dev_wallet(self) -> Self {
        let client = match self.anvil {
            // If anvil is running, use the private key at index 0.
            Some(ref anvil) => {
                let wallet: LocalWallet = anvil
                    .keys()
                    .get(0)
                    .expect("no keys in anvil")
                    .clone()
                    .into();
                let wallet = wallet.with_chain_id(anvil.chain_id());
                let url = anvil.endpoint();
                let url = url.replace("http", "ws");

                let provider = Provider::<Ws>::connect(&url)
                    .await
                    .expect("failed to connect to anvil");

                Arc::new(SignerMiddleware::new(provider, wallet))
            }
            None => panic!("anvil not running"),
        };

        Self {
            client: Some(client),
            ..self
        }
    }

    pub async fn with_counter(self) -> Self {
        let client = self.client.unwrap();
        let counter = Counter::deploy(client.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        let contract = counter.address();

        tracing::info!("Deployed counter contract at 0x{:x}", contract);

        Self {
            client: Some(client),
            counter_contract: Some(contract),
            ..self
        }
    }

    pub async fn with_coin(self) -> Self {
        let client = self.client.unwrap();
        let client_address = client.address();
        tracing::info!(
            "Deploying coin from address: 0x{:x}",
            client_address.clone()
        );
        let coin = Coin::deploy(client.clone(), ethers::utils::parse_ether("25").unwrap())
            .unwrap()
            .send()
            .await
            .unwrap();

        tracing::info!(
            "Client balance: {}",
            coin.balance_of(client_address.clone())
                .call()
                .await
                .unwrap(),
        );

        let coin = coin.address();

        tracing::info!("Deployed counter coin at 0x{:x}", coin);

        Self {
            client: Some(client),
            coin_contract: Some(coin),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_anvil() {
        let local = Local::<Ws>::default().with_anvil();
        assert!(local.anvil.is_some());
    }

    #[tokio::test]
    async fn test_with_dev_wallet() {
        let local = Local::<Ws>::default().with_anvil().with_dev_wallet().await;
        assert!(local.client.is_some());
    }
}
