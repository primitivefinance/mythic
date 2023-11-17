//! Handles simulation transactions that will go over live networks

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use anyhow::anyhow;
use arbiter_core::environment::{builder::EnvironmentBuilder, fork::ContractMetadata, Environment};
use ethers::{core::rand::thread_rng, prelude::*};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database,
};
use revm_primitives::AccountInfo;

use super::digest::{self, Artifacts};

pub struct Forker {
    pub environment: Environment,
    pub client: Option<Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>>,
    pub block_number: u64,
}

impl Default for Forker {
    fn default() -> Self {
        Self {
            environment: EnvironmentBuilder::new().build(),
            client: None,
            block_number: 2,
        }
    }
}

const RPC_URL_WS: &str = "ws://localhost:8545";
const CHAIN_ID: u64 = 31337;

/// Payload that is used for loading a target account's db info into a
/// `CacheDB`.
#[derive(Debug, Clone)]
pub struct IngestPayload {
    pub target: Address,
    pub artifacts_path: String,
}

impl From<IngestPayload> for ContractMetadata {
    fn from(payload: IngestPayload) -> Self {
        Self {
            address: payload.target,
            mappings: HashMap::new(),
            artifacts_path: payload.artifacts_path,
        }
    }
}

impl Forker {
    pub fn new(
        environment: Environment,
        client: Option<Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>>,
        block_number: u64,
    ) -> Self {
        Self {
            environment,
            client,
            block_number,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn with_block_number(mut self, block_number: u64) -> Self {
        tracing::debug!("Setting block number to {}", block_number);

        self.block_number = block_number;
        self
    }

    #[tracing::instrument(skip(wallet))]
    pub async fn connect(url: Option<String>, wallet: Option<LocalWallet>) -> anyhow::Result<Self> {
        // connect to the network
        let provider = match url {
            Some(url) => {
                // Replace http with ws
                let url = url.replace("http", "ws");

                tracing::info!("Connecting to network at url {}", url);
                Provider::<Ws>::connect(&url).await?
            }
            None => {
                tracing::info!("Connecting to network at url {}", RPC_URL_WS);
                Provider::<Ws>::connect(RPC_URL_WS).await?
            }
        };

        // make a wallet to use
        let wallet = match wallet {
            Some(wallet) => wallet.with_chain_id(CHAIN_ID),
            None => {
                // Get private key from env variable
                let pk = std::env::var("PRIVATE_KEY_DEV");
                match pk {
                    Ok(pk) => pk.parse::<LocalWallet>()?.with_chain_id(CHAIN_ID),
                    Err(_) => LocalWallet::new(&mut thread_rng()),
                }
            }
        };

        // connect the wallet to the provider
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self::new(
            EnvironmentBuilder::new().build(),
            Some(client.clone()),
            1,
        ))
    }

    #[tracing::instrument(skip(self))]
    pub fn spawn_ethers_db(&self) -> Result<EthersDB<Provider<Ws>>, anyhow::Error> {
        let client = self.client.clone();
        tracing::info!("Spawning db...");

        match client {
            Some(client) => {
                let provider = Arc::new(client.provider().clone());
                let ethers_db = EthersDB::new(
                    provider,
                    Some(BlockId::Number(BlockNumber::Number(
                        self.block_number.into(),
                    ))),
                )
                .unwrap();

                tracing::info!("Spawned db.");

                Ok(ethers_db)
            }
            None => Err(anyhow::anyhow!("No client")),
        }
    }

    /// Digests the config file and takes in an `EthersDB` so that the data can
    /// be fetched from the blockchain.
    /// Once all the `AccountInfo` for the contracts are fetched, we digest the
    /// contract artifacts to get the storage layout.
    #[tracing::instrument(skip(self))]
    pub fn digest_config(&self, addy: Address) -> anyhow::Result<CacheDB<EmptyDB>, anyhow::Error> {
        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let ethers_db = &mut self.spawn_ethers_db()?;
        tracing::info!("Digesting starting now...");

        // return Err(anyhow::anyhow!("Not implemented"));

        let mut db = CacheDB::new(EmptyDB::new());
        let mut contracts_meta = HashMap::new();

        let current_dir = std::env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        let path = std::path::Path::new(parent_dir)
            .join("box-contracts")
            .join("out")
            .join("Counter.sol")
            .join("counter.json")
            .to_str()
            .unwrap()
            .to_string();

        tracing::info!("Path: {}", path);

        let counter_contract_meta: ContractMetadata = ContractMetadata {
            address: addy,
            mappings: HashMap::new(),
            artifacts_path: path,
        };

        contracts_meta.insert("counter".to_string(), counter_contract_meta);

        tracing::info!("Amount of values to digest: {}", contracts_meta.len());

        for contract_data in contracts_meta.values() {
            tracing::info!("fetching account info for {:?}", contract_data.address);
            let address = contract_data.address;

            // Load account information
            let info = ethers_db
                .basic(address.to_fixed_bytes().into())
                .map_err(|_| {
                    anyhow!("Failed to fetch account info with
                EthersDB."
                        .to_string(),)
                })?
                .ok_or(anyhow!(
                    "Failed to fetch account info with EthersDB.".to_string(),
                ))?;

            tracing::info!("Account info: {:?}", info);

            db.insert_account_info(address.to_fixed_bytes().into(), info);

            // Load account storage
            let artifacts = digest::digest_artifacts(contract_data.artifacts_path.as_str())?;
            let storage_layout = artifacts.storage_layout;
            digest::create_storage_layout(contract_data, storage_layout, &mut db, ethers_db)?;
        }

        Ok(db)
    }

    #[tracing::instrument(skip(self))]
    fn fetch_account_info(
        &self,
        payload: IngestPayload,
    ) -> anyhow::Result<AccountInfo, anyhow::Error> {
        let provider = Arc::new(self.client.clone().unwrap().provider().clone());
        let start_block = BlockId::Number(BlockNumber::Number(self.block_number.into()));

        // Load account information from its own thread.
        let handle = std::thread::spawn(move || {
            let mut ethers_db = EthersDB::new(provider, Some(start_block)).unwrap();

            tracing::info!("fetching account info for {:?}", payload.target);
            let info = ethers_db
                .basic(payload.target.to_fixed_bytes().into())
                .map_err(|_| {
                    anyhow!("Failed to fetch account info with
                EthersDB."
                        .to_string(),)
                })
                .unwrap()
                .ok_or(anyhow!(
                    "Failed to fetch account info with EthersDB.".to_string(),
                ))
                .unwrap();

            info
        });

        let info = handle.join();

        tracing::debug!("Success Account info: {:?}", info);
        let info = match info {
            Ok(info) => info,
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                panic!("Error: {:?}", e);
            }
        };

        Ok(info)
    }

    #[tracing::instrument(skip(self))]
    fn fetch_artifacts(&self, payload: IngestPayload) -> anyhow::Result<Artifacts, anyhow::Error> {
        let artifacts = digest::digest_artifacts(payload.artifacts_path.as_str())?;

        Ok(artifacts)
    }

    #[tracing::instrument(skip(self))]
    fn load_cached_db(
        &self,
        payload: IngestPayload,
    ) -> anyhow::Result<CacheDB<EmptyDB>, anyhow::Error> {
        let mut db = CacheDB::new(EmptyDB::new());

        let info = self.fetch_account_info(payload.clone())?;
        db.insert_account_info(payload.target.to_fixed_bytes().into(), info);

        tracing::debug!("Fetching storage layout...");

        let artifacts = self.fetch_artifacts(payload.clone())?;
        let storage_layout = artifacts.storage_layout;
        digest::create_storage_layout(
            &payload.into(),
            storage_layout,
            &mut db,
            &mut self.spawn_ethers_db()?,
        )?;

        tracing::debug!("Storage layout fetched.");

        Ok(db)
    }

    /// Overrides `environment` with a database that was loaded from an
    /// `IngestPayload.
    #[tracing::instrument(skip(self))]
    pub fn evolve(mut self, payload: IngestPayload) -> Self {
        let db = self.load_cached_db(payload).unwrap();

        let _ = self.environment.stop();

        self.environment = EnvironmentBuilder::new().db(db.clone()).build();
        tracing::debug!("Environment evolved with db: {:?}", db.clone());
        self
    }
}

fn get_counter_path() -> anyhow::Result<std::path::PathBuf, anyhow::Error> {
    let current_dir = std::env::current_dir().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    let path = std::path::Path::new(parent_dir)
        .join("box-contracts")
        .join("out")
        .join("Counter.sol")
        .join("counter.json");

    Ok(path)
}

#[cfg(test)]
mod tests {
    use arbiter_core::middleware::RevmMiddleware;
    use ethers::{prelude::*, utils::Anvil};
    use simulation::bindings::counter::Counter;

    use super::*;
    use crate::mvp::api::tests::TEST_SUBSCRIBER;

    #[tokio::test]
    async fn test_spawn_ethers_db() -> anyhow::Result<(), anyhow::Error> {
        let _ = *TEST_SUBSCRIBER;
        let battler = Forker::connect(None, None).await?;
        let ethers_db = battler.spawn_ethers_db()?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_evolve() -> anyhow::Result<(), anyhow::Error> {
        let _ = *TEST_SUBSCRIBER;

        // Start anvil in the background.
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        // Load the dev wallet from anvil and have the Forker make a client with it.
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let battler = Forker::connect(Some(anvil.endpoint()), Some(wallet)).await?;

        // Deploy the counter contract.
        let client = battler.client.clone().unwrap();
        let counter = Counter::deploy(client.clone(), ())?.send().await?;
        let counter_address = counter.address();

        // Increment the counter so it's one.
        // This also increments the block to 1.
        let _ = counter.increment().send().await?.await?;

        // Assert the count is one, and get the block number.
        let count = counter.number().call().await?;
        assert_eq!(count, 1_u64.into());

        let block_number = client.get_block_number().await?;
        tracing::debug!("Block number: {}", block_number);

        // Create an ingest payload to load the counter contract into the database.
        let payload = IngestPayload {
            target: counter_address,
            artifacts_path: get_counter_path().unwrap().to_str().unwrap().to_string(),
        };

        // Evolve the Forker with the payload.
        let battler = battler
            .with_block_number(block_number.as_u64())
            .evolve(payload);

        // Get the arbiter client to talk to the evolved Forker.
        let arbiter_client = RevmMiddleware::new(&battler.environment, Some("evolve"))?;

        // Get the Counter contract in Arbiter.
        let counter_arbiter = Counter::new(counter_address, arbiter_client.clone());

        // Call the counter's current count in the arbiter environment.
        let count = counter_arbiter.number().call().await?;

        // Check that the count is one.
        assert_eq!(count, 1_u64.into());

        // Increment the counter again so it's two, but in arbiter.
        tracing::debug!("Incrementing counter in arbiter...");
        let _ = counter_arbiter.increment().send().await?.await?.unwrap();

        // Call the counter's current count in the arbiter environment.
        let count = counter_arbiter.number().call().await?;

        // Check that the count is two.
        assert_eq!(count, 2_u64.into());
        tracing::debug!("Final count: {}", count);

        Ok(())
    }

    /// Is anvil in your user path?
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_digest_into_db() -> anyhow::Result<(), anyhow::Error> {
        let _ = *TEST_SUBSCRIBER;
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();
        tracing::info!(
            "Anvil spawned at endpoint {} with chain {}",
            anvil.endpoint(),
            anvil.chain_id()
        );
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let battler = Forker::connect(Some(anvil.endpoint()), Some(wallet)).await?;

        let client = battler.client.clone().unwrap();
        let counter = Counter::deploy(client.clone(), ())?.send().await?;
        let counter_address = counter.address();
        tracing::info!("Counter address: {}", counter_address.clone());

        // VERY IMPORTANT
        let handle = std::thread::spawn(move || {
            let cached = match battler.digest_config(counter_address) {
                Ok(cached) => cached,
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                    panic!("Error: {:?}", e);
                }
            };
            cached
        });

        let mut cached = handle.join().unwrap();
        let loaded = cached
            .load_account(counter_address.to_fixed_bytes().into())
            .unwrap();
        tracing::info!("Loaded: {:?}", loaded);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_storage_mutation() -> anyhow::Result<(), anyhow::Error> {
        let _ = *TEST_SUBSCRIBER;
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();
        tracing::info!(
            "Anvil spawned at endpoint {} with chain {}",
            anvil.endpoint(),
            anvil.chain_id()
        );
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();
        let mut battler = Forker::connect(Some(anvil.endpoint()), Some(wallet)).await?;

        let client = battler.client.clone().unwrap();
        let counter = Counter::deploy(client.clone(), ())?.send().await?;
        let counter_address = counter.address();
        tracing::info!("Counter address: {}", counter_address.clone());

        let tx = counter.increment().send().await?.await?;
        tracing::info!("Tx: {:?}", tx);

        let battler = battler.with_block_number(2_u64);

        // VERY IMPORTANT
        let handle = std::thread::spawn(move || {
            let cached = battler.digest_config(counter_address).unwrap();
            (cached, battler)
        });

        let (mut cached, battler) = handle.join().unwrap();
        let acc_before = cached
            .load_account(counter_address.to_fixed_bytes().into())
            .unwrap();
        tracing::info!("Loaded before: {:?}", acc_before);

        // increment the counter
        let tx = counter.increment().send().await?.await?;
        tracing::info!("Tx: {:?}", tx);

        let count = counter.number().call().await?;
        tracing::info!("Count: {:?}", count);

        let battler = battler.with_block_number(3_u64);

        // re-digest
        let handle = std::thread::spawn(move || {
            let cached = match battler.digest_config(counter_address) {
                Ok(cached) => cached,
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                    panic!("Error: {:?}", e);
                }
            };

            cached
        });

        let mut cached = handle.join().unwrap();

        let acc_after = cached
            .load_account(counter_address.to_fixed_bytes().into())
            .unwrap();

        tracing::info!("Loaded after: {:?}", acc_after);

        let count_from_storage = acc_after
            .storage
            .get(&revm::primitives::U256::ZERO)
            .unwrap();
        assert_eq!(*count_from_storage, revm::primitives::U256::from(2_u64));

        Ok(())
    }
}
