use ethers::prelude::*;
use ethers::utils::{Anvil, AnvilInstance};
use std::sync::Arc;
use std::{collections::HashMap, fmt};

use crate::app::AnvilSave;

pub const SANDBOX_LABEL: &str = "sandbox";

pub type NetworkClient<P, S> = SignerMiddleware<Provider<P>, S>;

pub struct ExcaliburMiddleware<P: PubsubClient, S: Signer> {
    pub client: Option<Arc<NetworkClient<P, S>>>,
    pub signer: Option<S>,
    pub contracts: HashMap<String, Address>,
    pub anvil: Option<AnvilInstance>,
}

impl fmt::Debug for ExcaliburMiddleware<Ws, LocalWallet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("client", &self.client)
            .finish()
    }
}

impl<P: PubsubClient, S: Signer> ExcaliburMiddleware<P, S> {
    pub fn get_client(&self) -> Arc<NetworkClient<P, S>> {
        self.client.as_ref().unwrap().clone()
    }

    pub fn address(&self) -> Option<Address> {
        self.signer.as_ref().map(|signer| signer.address())
    }

    pub fn add_contract(&mut self, name: &str, address: Address) {
        self.contracts.insert(name.to_string(), address);
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_ledger(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn snapshot(&self) -> anyhow::Result<AnvilSave> {
        if self.anvil.is_some() {
            let block_number = self
                .client
                .as_ref()
                .unwrap()
                .provider()
                .get_block_number()
                .await?
                .as_u64();

            let params: Vec<String> = vec![];
            let snapshot = self
                .client
                .as_ref()
                .unwrap()
                .provider()
                .request("anvil_dumpState", params)
                .await
                .expect("failed to get snapshot");

            Ok(AnvilSave {
                block_number,
                snapshot,
            })
        } else {
            Err(anyhow::anyhow!("No anvil instance set."))
        }
    }
}

impl ExcaliburMiddleware<Ws, LocalWallet> {
    pub async fn new(
        anvil: Option<AnvilInstance>,
        signer: Option<LocalWallet>,
    ) -> anyhow::Result<Self> {
        let mut anvil_client = None;
        if let Some(anvil_instance) = anvil.as_ref() {
            let signer = signer
                .clone()
                .unwrap_or_else(|| LocalWallet::from(anvil_instance.keys()[0].clone()));

            anvil_client = Some(Arc::new(
                Provider::<Ws>::connect(&anvil_instance.ws_endpoint())
                    .await?
                    .with_signer(signer.with_chain_id(anvil_instance.chain_id())),
            ));
        }

        let mut client = None;
        if let Some(anvil_client) = anvil_client.clone() {
            client = Some(anvil_client);
        }

        Ok(Self {
            client,
            signer,
            contracts: HashMap::new(),
            anvil,
        })
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_ws(&mut self, endpoint: &str, chain_id: u64) -> anyhow::Result<()> {
        let signer = self.signer.clone().unwrap();
        let client = Arc::new(
            Provider::<Ws>::connect(endpoint)
                .await?
                .interval(std::time::Duration::from_millis(100))
                .with_signer(signer.with_chain_id(chain_id)),
        );

        self.client = Some(client);
        Ok(())
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_signer(&mut self, signer: LocalWallet) -> anyhow::Result<()> {
        let provider = self.client.as_ref().unwrap().provider().clone();
        let chain_id = provider.get_chainid().await?.as_u64();
        let signer = signer.with_chain_id(chain_id);

        let signer_client = Arc::new(
            provider
                .interval(std::time::Duration::from_millis(10))
                .with_signer(signer),
        );
        self.client = Some(signer_client.clone());

        Ok(())
    }

    #[tracing::instrument(skip(self, anvil), level = "debug")]
    pub async fn connect_anvil(&mut self, anvil: AnvilInstance) -> anyhow::Result<()> {
        let signer = LocalWallet::from(anvil.keys()[0].clone());
        let client = Arc::new(
            Provider::<Ws>::connect(&anvil.ws_endpoint())
                .await?
                .interval(std::time::Duration::from_millis(10))
                .with_signer(signer.clone().with_chain_id(anvil.chain_id())),
        );

        self.anvil = Some(anvil);
        self.client = Some(client);
        self.signer = Some(signer);

        Ok(())
    }
}

pub fn start_anvil(chain_id: Option<u64>) -> anyhow::Result<AnvilInstance> {
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let binary_path = format!("{}/.foundry/bin/anvil", home_dir);

    let chain_id = chain_id.unwrap_or(31337_u64);
    let anvil = Anvil::default()
        .arg("--gas-limit")
        .arg("20000000")
        .chain_id(chain_id)
        .path(binary_path)
        .spawn();

    Ok(anvil)
}

#[cfg(test)]
mod tests {

    use ethers::utils::{format_ether, Anvil};

    use super::*;
    #[allow(dead_code)]
    async fn setup() -> anyhow::Result<AnvilInstance> {
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        Ok(anvil)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_excalibur_local() -> anyhow::Result<()> {
        let client = ExcaliburMiddleware::new(None, None).await?;

        let anvil = client.anvil.as_ref().unwrap();

        let bob: LocalWallet = anvil
            .keys()
            .first()
            .expect("no keys in anvil")
            .clone()
            .into();

        let alice_wallet = LocalWallet::from(anvil.keys()[1].clone());
        let bob = bob.with_chain_id(anvil.chain_id());

        let alice_address = alice_wallet.address();
        let bob_address = bob.address();

        let balance = client
            .get_client()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .get_client()
            .get_balance(alice_address, None)
            .await
            .unwrap();

        println!("balance: {}", format_ether(balance));
        println!("alice balance: {}", format_ether(alice_balance));

        let _ = TransactionRequest::pay(
            alice_address,
            ethers::types::U256::from(1_000_000_000_000_000_000u128),
        );

        let balance = client
            .get_client()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .get_client()
            .get_balance(alice_address, None)
            .await
            .unwrap();

        println!("balance: {}", format_ether(balance));
        println!("alice balance: {}", format_ether(alice_balance));

        Ok(())
    }
}
