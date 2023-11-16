//! Handles simulation transactions that will go over live networks

use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use arbiter_core::environment::{builder::EnvironmentBuilder, fork::ContractMetadata, Environment};
use ethers::{
    core::{k256::Secp256k1, rand::thread_rng},
    prelude::*,
};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database,
};
use serde::{Deserialize, Serialize};

use super::digest;

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

    pub fn with_block_number(mut self, block_number: u64) -> Self {
        self.block_number = block_number;
        self
    }

    pub async fn connect(url: Option<String>, wallet: Option<LocalWallet>) -> anyhow::Result<Self> {
        // connect to the network
        let provider = match url {
            Some(url) => {
                // Replace http with ws
                let url = url.replace("http", "ws");

                Provider::<Ws>::connect(&url).await?
            }
            None => Provider::<Ws>::connect(RPC_URL_WS).await?,
        };
        tracing::info!("Connected to network at url {}", RPC_URL_WS);

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
}

#[cfg(test)]
mod tests {
    use ethers::{prelude::*, utils::Anvil};
    use simulation::bindings::counter::Counter;

    use super::*;

    #[tokio::test]
    async fn test_spawn_ethers_db() -> anyhow::Result<(), anyhow::Error> {
        let battler = Forker::connect(None, None).await?;
        let ethers_db = battler.spawn_ethers_db()?;

        Ok(())
    }

    /// Is anvil in your user path?
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_digest_into_db() -> anyhow::Result<(), anyhow::Error> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
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
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
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
