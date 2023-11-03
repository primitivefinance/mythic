//! Abstraction for interacting with production or test networks via an
//! ethers-rs provider/client.

use std::sync::Arc;

use ethers::prelude::*;
use tracing::info;

const RPC_URL_WS: &str = "ws://localhost:8545";
const CHAIN_ID: u64 = 31337;

#[derive(Debug, Clone)]
pub struct Production<C> {
    pub client: Arc<SignerMiddleware<Provider<C>, LocalWallet>>,
}

impl Production<Ws> {
    pub async fn new() -> eyre::Result<Self> {
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
            client: client.clone(),
        })
    }

    /// Used for testing the connection to the network.
    /// Make sure the rpc is online, i.e. if using localhost anvil must be
    /// running.
    pub async fn debug_tx(&self) -> eyre::Result<()> {
        let client = self.client.clone();
        let tx = TransactionRequest::new().to(client.address()).value(10000);

        // send it!
        let pending_tx = client.send_transaction(tx, None).await?;

        // get the mined tx
        let receipt = pending_tx
            .await?
            .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;
        let tx = client.get_transaction(receipt.transaction_hash).await?;

        info!("Sent tx: {}\n", serde_json::to_string(&tx)?);
        info!("Tx receipt: {}", serde_json::to_string(&receipt)?);

        Ok(())
    }

    /// Returns a clone of the provider.
    pub fn get(&self) -> Arc<Provider<Ws>> {
        let provider = self.client.provider().clone();
        Arc::new(provider)
    }
}

#[cfg(test)]
mod tests {

    use dotenv::dotenv;
    use ethers::prelude::*;

    /// Fails if anvil is not running!
    #[tokio::test]
    async fn test_production() -> eyre::Result<()> {
        dotenv().ok();
        let production = crate::sdk::production::Production::new().await?;
        let client = production.client.clone();
        let balance = client.get_balance(client.address(), None).await?;
        println!("Balance: {}", balance);

        // do debug tx
        let _ = production.debug_tx().await?;

        // check balance again
        let balance2 = client.get_balance(client.address(), None).await?;

        // assert balance changed
        assert!(balance2 != balance);

        Ok(())
    }
}
