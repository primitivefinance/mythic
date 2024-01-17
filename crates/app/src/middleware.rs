use std::{collections::HashMap, fmt};

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
use clients::ledger::LedgerClient;
use ethers::utils::{Anvil, AnvilInstance};

use super::*;
use crate::{app::AnvilSave, model::portfolio::EthersAddress};

pub const SANDBOX_LABEL: &str = "sandbox";

/// Standard client that excalibur uses.
pub type NetworkClient<P, S> = SignerMiddleware<Provider<P>, S>;

/// Connects users to networks.
pub struct ExcaliburMiddleware<P: PubsubClient, S: Signer> {
    /// ARBITER
    pub sandbox: Environment,
    /// ARBITER
    pub sandbox_client: Arc<RevmMiddleware>,
    /// ANVIL
    pub anvil: Option<AnvilInstance>,
    /// ANVIL
    pub anvil_client: Option<Arc<SignerMiddleware<Provider<P>, S>>>,
    /// ANY
    pub clients: Vec<Arc<SignerMiddleware<Provider<P>, S>>>,
    /// ANY
    pub signers: Vec<S>,
    /// ANY
    pub contracts: HashMap<String, EthersAddress>,
    /// HARDWARE
    pub ledger: Option<LedgerClient>,
}

impl fmt::Debug for ExcaliburMiddleware<Ws, LocalWallet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("sandbox_client", &self.sandbox_client)
            .field("anvil_client", &self.anvil_client)
            .field("clients", &self.clients)
            .field("signers", &self.signers)
            .finish()
    }
}

impl<P: PubsubClient, S: Signer> ExcaliburMiddleware<P, S> {
    pub fn add_contract(&mut self, name: &str, address: EthersAddress) {
        self.contracts.insert(name.to_string(), address);
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_ledger(&mut self) -> anyhow::Result<()> {
        let ledger =
            LedgerClient::new_connection(clients::ledger::types::DerivationType::LedgerLive(0))
                .await;

        let ledger = match ledger {
            Ok(ledger) => Some(ledger),
            Err(e) => {
                tracing::warn!("Could not connect to ledger: {:?}", e);
                None
            }
        };

        self.ledger = ledger;
        Ok(())
    }

    /// Returns the sandbox environment client.
    pub fn sandbox(&self) -> Arc<RevmMiddleware> {
        self.sandbox_client.clone()
    }

    /// Returns the first provider in the list.
    pub fn client(&self) -> Option<&Arc<NetworkClient<P, S>>> {
        self.clients.first()
    }

    /// Returns the first signer in the list.
    pub fn signer(&self) -> Option<&S> {
        self.signers.first()
    }

    /// Returns the address of the signer, if there is one.
    pub fn address(&self) -> Option<Address> {
        self.signer().map(|signer| signer.address())
    }

    /// Executes the `anvil_dumpState` rpc call on the anvil instance.
    pub async fn snapshot(&self) -> anyhow::Result<AnvilSave> {
        let anvil_client = self.anvil_client.as_ref().unwrap();
        let block_number = anvil_client.provider().get_block_number().await?.as_u64();

        let params: Vec<String> = vec![];
        let snapshot = anvil_client
            .provider()
            .request("anvil_dumpState", params)
            .await
            .expect("failed to get snapshot");

        Ok(AnvilSave {
            block_number,
            snapshot,
        })
    }
}

impl ExcaliburMiddleware<Ws, LocalWallet> {
    pub async fn new(
        anvil: Option<AnvilInstance>,
        sandbox: Environment,
        signer: LocalWallet,
    ) -> anyhow::Result<Self> {
        let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;

        let mut anvil_client = None;
        if let Some(anvil_instance) = anvil.as_ref() {
            anvil_client = Some(Arc::new(
                Provider::<Ws>::connect(&anvil_instance.ws_endpoint())
                    .await?
                    .with_signer(signer.clone().with_chain_id(anvil_instance.chain_id())),
            ));
        }

        let signers = vec![signer];

        let mut clients = vec![];
        if let Some(anvil_client) = anvil_client.clone() {
            clients.push(anvil_client);
        }

        Ok(Self {
            sandbox,
            sandbox_client,
            anvil,
            anvil_client,
            clients,
            signers,
            contracts: HashMap::new(),
            ledger: None,
        })
    }

    /// Starts the sandbox environment.
    /// If `dev` is true, it will also start an anvil instance and connect a
    /// provider to it.
    pub async fn setup(dev: bool) -> anyhow::Result<Self> {
        if dev {
            let home_dir = std::env::var("HOME").unwrap_or_default();
            let binary_path = format!("{}/.foundry/bin/anvil", home_dir);
            let anvil = Anvil::default()
                .arg("--gas-limit")
                .arg("20000000")
                .chain_id(31337_u64)
                .path(binary_path)
                .spawn();

            let signer = LocalWallet::from(anvil.keys()[0].clone());
            let sandbox = EnvironmentBuilder::new().build();
            let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;
            let anvil_client = Arc::new(
                Provider::<Ws>::connect(&anvil.ws_endpoint())
                    .await?
                    .interval(std::time::Duration::from_millis(100))
                    .with_signer(signer.clone().with_chain_id(anvil.chain_id())),
            );
            let signers = vec![signer];
            let clients = vec![anvil_client.clone()];

            Ok(Self {
                sandbox,
                sandbox_client,
                anvil: Some(anvil),
                anvil_client: Some(anvil_client),
                clients,
                signers,
                contracts: HashMap::new(),
                ledger: None,
            })
        } else {
            let sandbox = EnvironmentBuilder::new().build();
            let sandbox_client = RevmMiddleware::new(&sandbox, Some(SANDBOX_LABEL))?;
            let clients = vec![];
            let signers = vec![];

            Ok(Self {
                anvil: None,
                anvil_client: None,
                sandbox,
                sandbox_client,
                clients,
                signers,
                contracts: HashMap::new(),
                ledger: None,
            })
        }
    }

    pub fn add_signer(&mut self, signer: LocalWallet) -> anyhow::Result<()> {
        self.signers.push(signer);
        Ok(())
    }
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
        let client = ExcaliburMiddleware::setup(true).await?;

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
            .client()
            .unwrap()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .client()
            .unwrap()
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
            .client()
            .unwrap()
            .get_balance(bob_address, None)
            .await
            .unwrap();

        let alice_balance = client
            .client()
            .unwrap()
            .get_balance(alice_address, None)
            .await
            .unwrap();

        println!("balance: {}", format_ether(balance));
        println!("alice balance: {}", format_ether(alice_balance));

        Ok(())
    }
}
