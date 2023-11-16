//! Handles simulation transactions that will go over live networks

use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use arbiter_core::environment::{builder::EnvironmentBuilder, Environment};
use ethers::{core::rand::thread_rng, prelude::*};
use revm::{
    db::{ethersdb::EthersDB, CacheDB, EmptyDB},
    Database,
};
use serde::{Deserialize, Serialize};

pub struct BattleTester {
    pub environment: Environment,
    pub client: Option<Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>>,
    pub block_number: u64,
}

impl Default for BattleTester {
    fn default() -> Self {
        Self {
            environment: EnvironmentBuilder::new().build(),
            client: None,
            block_number: 0,
        }
    }
}

const RPC_URL_WS: &str = "ws://localhost:8545";
const CHAIN_ID: u64 = 31337;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub address: ethers::types::Address,
    pub mappings: HashMap<String, Vec<String>>,
}

impl BattleTester {
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

    pub async fn connect() -> anyhow::Result<Self> {
        // connect to the network
        let provider = Provider::<Ws>::connect(RPC_URL_WS).await?;
        tracing::info!("Connected to network at url {}", RPC_URL_WS);

        // Get private key from env variable
        let pk = std::env::var("PRIVATE_KEY_DEV");

        // make a wallet to use
        let wallet = match pk {
            Ok(pk) => pk.parse::<LocalWallet>()?.with_chain_id(CHAIN_ID),
            Err(_) => LocalWallet::new(&mut thread_rng()),
        };

        // connect the wallet to the provider
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self::new(
            EnvironmentBuilder::new().build(),
            Some(client.clone()),
            0,
        ))
    }

    #[tracing::instrument(skip(self))]
    pub fn spawn_ethers_db(&self) -> Result<EthersDB<Provider<Ws>>, anyhow::Error> {
        let client = self.client.clone();

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
    pub(crate) fn digest_config(&self) -> anyhow::Result<CacheDB<EmptyDB>, anyhow::Error> {
        // Spawn the `EthersDB` and the `CacheDB` we will write to.
        let ethers_db = &mut self.spawn_ethers_db()?;
        let mut db = CacheDB::new(EmptyDB::default());
        for contract_data in self.contracts_meta.values() {
            let address = contract_data.address;
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

            db.insert_account_info(address.to_fixed_bytes().into(), info);
            let artifacts = digest::digest_artifacts(contract_data.artifacts_path.as_str())?;
            let storage_layout = artifacts.storage_layout;

            digest::create_storage_layout(contract_data, storage_layout, &mut db, ethers_db)?;

            for eoa in self.externally_owned_accounts.values() {
                let info = ethers_db
                    .basic(eoa.to_fixed_bytes().into())
                    .map_err(|_| {
                        anyhow!("Failed to fetch account info with
                EthersDB."
                            .to_string(),)
                    })?
                    .ok_or(anyhow!(
                        "Failed to fetch account info with EthersDB.".to_string(),
                    ))?;
                db.insert_account_info(eoa.to_fixed_bytes().into(), info);
            }
        }
        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use ethers::prelude::*;

    use super::*;

    #[tokio::test]
    async fn test_spawn_ethers_db() -> anyhow::Result<(), anyhow::Error> {
        let battler = BattleTester::connect().await?;
        let ethers_db = battler.spawn_ethers_db()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_load_account() -> anyhow::Result<(), anyhow::Error> {
        let battler = BattleTester::connect().await?;

        Ok(())
    }
}
