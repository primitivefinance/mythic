use std::{collections::HashMap, fmt};

use arbiter_core::{environment::Environment, middleware::ArbiterMiddleware};
use clients::{ledger::LedgerClient, protocol::ProtocolClient};
use ethers::utils::{Anvil, AnvilInstance};

use super::*;
use crate::{app::AnvilSave, model::portfolio::EthersAddress};

pub const SANDBOX_LABEL: &str = "sandbox";

/// Standard client that excalibur uses.
pub type NetworkClient<P, S> = SignerMiddleware<Provider<P>, S>;

/// Connects users to networks.
/// - Anvil instance is optional and can be connected via `connect_anvil`.
/// - Arbiter is optional and can be connected via `connect_arbiter`.
/// - Ledger is optional and can be connected via `connect_ledger`.
/// - Active client is the currently existing connection.
/// - Active signer is the currently existing signer (if there is one).
/// - Contracts are a stateful map of human readable contract identifiers to
///   addresses.
/// - note: if AnvilInstance is some, then the client is the client for Anvil.
pub struct ExcaliburMiddleware<P: PubsubClient, S: Signer> {
    /// ACTIVE CLIENT CONNECTION
    pub client: Option<Arc<NetworkClient<P, S>>>,
    /// ACTIVE SIGNER
    pub signer: Option<S>,
    /// ANY CONTRACTS
    pub contracts: HashMap<String, EthersAddress>,
    /// HARDWARE
    pub ledger: Option<LedgerClient>,
    /// ARBITER
    pub arbiter: Option<Environment>,
    /// ARBITER CLIENT
    pub arbiter_client: Option<Arc<ArbiterMiddleware>>,
    /// ANVIL
    pub anvil: Option<AnvilInstance>,
    /// PROTOCOL
    pub dfmm_client: Option<ProtocolClient<NetworkClient<P, S>>>,
}

impl fmt::Debug for ExcaliburMiddleware<Ws, LocalWallet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("client", &self.client)
            .finish()
    }
}

impl<P: PubsubClient, S: Signer> ExcaliburMiddleware<P, S> {
    /// Returns a ref to the unwrapped client
    pub fn get_client(&self) -> Arc<NetworkClient<P, S>> {
        self.client.as_ref().unwrap().clone()
    }

    /// Returns the address of the signer, if there is one.
    pub fn address(&self) -> Option<Address> {
        self.signer.as_ref().map(|signer| signer.address())
    }

    /// Adds a new contract to the contracts map.
    pub fn add_contract(&mut self, name: &str, address: EthersAddress) {
        self.contracts.insert(name.to_string(), address);
    }

    /// Creates a connection to a ledger device.
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

    /// Connects the middleware to an arbiter `Environment`.
    /// Must use the method `arbiter_client` field to make
    /// blockchain calls to arbiter with.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_arbiter(
        &mut self,
        arbiter: Environment,
        seed: Option<&str>,
    ) -> anyhow::Result<()> {
        let arbiter_client = ArbiterMiddleware::new(&arbiter, seed)?;

        self.arbiter = Some(arbiter);
        self.arbiter_client = Some(arbiter_client.clone());

        Ok(())
    }

    /// Executes the `anvil_dumpState` rpc call on the anvil instance.
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
    /// Creates a new Excalibur middleware instance, setting the anvil and/or
    /// arbiter instances if provided.
    /// - If anvil is available, then the client is automatically connected to
    ///   an anvil provider.
    /// - If arbiter is available, then the the arbiter client is automatically
    ///   connected and available to use.
    /// - Else, no client is connected and it must be connected to while the app
    ///   is running.
    pub async fn new(
        anvil: Option<AnvilInstance>,
        arbiter: Option<Environment>,
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

        let mut arbiter_client = None;
        if let Some(arbiter_instance) = arbiter.as_ref() {
            arbiter_client = Some(ArbiterMiddleware::new(
                arbiter_instance,
                Some(SANDBOX_LABEL),
            )?);
        }

        let mut client = None;
        if let Some(anvil_client) = anvil_client.clone() {
            client = Some(anvil_client);
        }

        Ok(Self {
            client,
            signer,
            contracts: HashMap::new(),
            ledger: None,
            arbiter,
            arbiter_client,
            anvil,
            dfmm_client: None,
        })
    }

    /// Connects to a network via a websocket endpoint.
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

    /// Connects a signer to the client.
    /// todo: replacing the client like this... are there side effects?
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

        // Override the dfmm_client if it exists with the new signer.
        if let Some(dfmm_client) = self.dfmm_client.as_ref() {
            self.dfmm_client = Some(dfmm_client.clone().connect(signer_client.clone())?);
        }

        Ok(())
    }

    /// Connects the middleware to a running anvil instance.
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

    /// Connects the middleware to the dfmm protocol client.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_dfmm(
        &mut self,
        client: ProtocolClient<NetworkClient<Ws, LocalWallet>>,
    ) -> anyhow::Result<()> {
        self.dfmm_client = Some(client);
        Ok(())
    }
}

/// Spawns a new anvil instance.
/// note: Requires anvil to be installed and available in the path.
pub fn start_anvil(chain_id: Option<u64>) -> anyhow::Result<AnvilInstance> {
    let home_dir = std::env::var("HOME").unwrap_or_default();
    let binary_path = format!("{}/.foundry/bin/anvil", home_dir);

    // todo: throw an error if the binary doesn't exist

    let chain_id = chain_id.unwrap_or(31337_u64);
    let anvil = Anvil::default()
        .arg("--gas-limit")
        .arg("20000000")
        .chain_id(chain_id)
        .path(binary_path)
        .spawn();

    Ok(anvil)
}

/// Spawns a new Arbiter instance.
#[allow(dead_code)]
pub fn start_arbiter() -> anyhow::Result<Environment> {
    let arbiter = Environment::builder().build();

    Ok(arbiter)
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
        let client = ExcaliburMiddleware::new(None, None, None).await?;

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
