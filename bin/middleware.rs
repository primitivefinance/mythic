use std::{collections::HashMap, fmt};

use arbiter_core::{environment::Environment, middleware::ArbiterMiddleware};
use clients::{ledger::LedgerClient, protocol::ProtocolClient};
use dfmm::portfolio::Portfolio;
use bindings::dfmm::DFMM;
use ethers::utils::{Anvil, AnvilInstance};

use super::*;
use crate::app::StateCache;

pub const SANDBOX_LABEL: &str = "sandbox";

/// Standard client that excalibur uses.
/// Connects users to networks.
/// - Anvil instance is optional and can be connected via `connect_anvil`.
/// - Arbiter is optional and can be connected via `connect_arbiter`.
/// - Ledger is optional and can be connected via `connect_ledger`.
/// - Active client is the currently existing connection.
/// - Active signer is the currently existing signer (if there is one).
/// - Contracts are a stateful map of human readable contract identifiers to
///   addresses.
/// - note: if AnvilInstance is some, then the client is the client for Anvil.
pub struct ExcaliburMiddleware {
    /// ANY CONTRACTS
    pub contracts: HashMap<String, Address>,
    /// HARDWARE
    pub ledger: Option<LedgerClient>,
    /// ARBITER
    pub arbiter: Environment,
    /// ARBITER CLIENT
    pub arbiter_client: Arc<ArbiterMiddleware>,
}

impl fmt::Debug for ExcaliburMiddleware {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("client", &self.client)
            .finish()
    }
}

impl ExcaliburMiddleware {
    /// Returns a ref to the unwrapped client
    pub fn get_client(&self) -> ArbiterMiddleware {
        self.client.as_ref().unwrap().clone()
    }

    /// Returns the address of the signer, if there is one.
    pub fn address(&self) -> Option<Address> {
        self.signer.as_ref().map(|signer| signer.address())
    }

    /// Adds a new contract to the contracts map.
    pub fn add_contract(&mut self, name: &str, address: Address) {
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

    /// Executes the `anvil_dumpState` rpc call on the anvil instance.
    pub async fn snapshot(&self) -> anyhow::Result<StateCache> {
        if self.arbiter_client.is_some() {
            let block_number = self
                .arbiter_client
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

            Ok(StateCache {
                block_number,
                snapshot,
            })
        } else {
            Err(anyhow::anyhow!("No anvil instance set."))
        }
    }
}

impl ExcaliburMiddleware {
    /// Creates a new Excalibur middleware instance, setting the anvil and/or
    /// arbiter instances if provided.
    /// - If anvil is available, then the client is automatically connected to
    ///   an anvil provider.
    /// - If arbiter is available, then the the arbiter client is automatically
    ///   connected and available to use.
    /// - Else, no client is connected and it must be connected to while the app
    ///   is running.
    pub async fn new(
    ) -> anyhow::Result<Self> {

        let env = Environment::builder().build();
        let arbiter_client = Some(ArbiterMiddleware::new(
            env.as_ref().unwrap(),
            Some(SANDBOX_LABEL),)?);


        Ok(Self {
            contracts: HashMap::new(),
            ledger: None,
            arbiter: env,
            arbiter_client,
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
        Ok(())
    }

    pub async fn create_dfmm_position(&mut self) {
        let address = self.contracts.get("dfmm").unwrap();


        let dfmm = DFMM::new(address, self.client.clone().unwrap());
        let position = dfmm.create_position().await?;
        Ok(position)
    
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