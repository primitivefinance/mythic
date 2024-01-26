use std::{collections::HashMap, fmt};

use arbiter_core::{
    environment::{builder::EnvironmentBuilder, Environment},
    middleware::RevmMiddleware,
};
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
pub struct ExcaliburMiddleware<M: Middleware + Signer + PubsubClient, S: Signer> {
    /// ACTIVE CLIENT CONNECTION
    pub client: Option<Arc<M>>,
    /// ACTIVE SIGNER
    pub signer: Option<S>,
    /// ANY CONTRACTS
    pub contracts: HashMap<String, EthersAddress>,
    /// HARDWARE
    pub ledger: Option<LedgerClient>,
    /// ARBITER
    pub arbiter: Option<Environment>,
    /// ARBITER CLIENT
    pub arbiter_client: Option<Arc<RevmMiddleware>>,
    /// ANVIL
    pub anvil: Option<AnvilInstance>,
    /// PROTOCOL
    pub dfmm_client: Option<ProtocolClient<M>>,
}

impl<M: Middleware + Signer + PubsubClient, S: Signer> std::fmt::Debug
    for ExcaliburMiddleware<M, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExcaliburMiddleware")
            .field("client", &self.client)
            .finish()
    }
}

impl<M: Middleware + Signer + PubsubClient, S: Signer> ExcaliburMiddleware<M, S> {
    /// Returns a ref to the unwrapped client
    pub fn get_client(&self) -> Arc<M> {
        self.client.as_ref().unwrap().clone()
    }

    /// Returns the address of the signer, if there is one.
    pub fn address(&self) -> Option<Address> {
        self.signer
            .as_ref()
            .map(|signer| signer.address())
            .or_else(|| self.client.as_ref().map(|client| client.address()))
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
        let arbiter_client = RevmMiddleware::new(&arbiter, seed)?;

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

impl ExcaliburMiddleware<RevmMiddleware, RevmMiddleware> {
    /// Creates a new Excalibur middleware instance, setting the anvil and/or
    /// arbiter instances if provided.
    /// - If anvil is available, then the client is automatically connected to
    ///   an anvil provider.
    /// - If arbiter is available, then the the arbiter client is automatically
    ///   connected and available to use.
    /// - Else, no client is connected and it must be connected to while the app
    ///   is running.
    pub async fn new(
        _anvil: Option<AnvilInstance>,
        arbiter: Option<Environment>,
        signer: Option<RevmMiddleware>,
    ) -> anyhow::Result<Self> {
        let mut environment = arbiter;
        if environment.is_none() {
            environment = Some(EnvironmentBuilder::new().build());
        }

        let arbiter_client = RevmMiddleware::new(environment.as_ref().unwrap(), None)?;

        Ok(Self {
            client: Some(arbiter_client.clone()),
            signer: None,
            contracts: HashMap::new(),
            ledger: None,
            arbiter: environment,
            arbiter_client: None,
            anvil: None,
            dfmm_client: None,
        })
    }

    /// Connects the middleware to the dfmm protocol client.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn connect_dfmm(
        &mut self,
        client: ProtocolClient<RevmMiddleware>,
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
    let arbiter = EnvironmentBuilder::new().build();

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

    #[cfg(feature = "anvil")]
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
