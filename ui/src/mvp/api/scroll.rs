//! A [`Scroll`] is a transaction over its entire lifetime.
//! 1. Crafting the transaction. Build its target, payload, amounts, gas
//!    settings, etc.
//! 2. Seal the payload. Sealing the payload lets the application know to only
//!    handle modifications
//! if we revert to the crafting phase.
//! 3. Load the target account's database information. This is used to compare
//!    the account's storage slots,
//! before and after a transaction.
//! 4. Load the database into an Arbiter environment instance.
//! 5. Execute the payload on the Arbiter instance, using the loaded database.
//! 6. Compare the storage slots before and after the transaction.
//! 7. Finally, execute the transaction.

use std::{convert::TryFrom, fs::File, io::BufReader, path::PathBuf};

use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::RevmMiddleware};
use ethers::{
    abi::Token,
    types::{transaction::eip2718::TypedTransaction, Address},
};
use revm::db::{CacheDB, EmptyDB};
use tokio::task::JoinHandle;

use super::{
    forking::{Forker, IngestPayload},
    *,
};

#[derive(Default, Debug, Clone)]
pub struct Stages {
    pub before: Option<CacheDB<EmptyDB>>,
    pub after: Option<CacheDB<EmptyDB>>,
}

#[derive(Default, Debug, Clone)]
pub struct Outcome {
    pub tx_hash: H256,
    pub receipt: TransactionReceipt,
}

#[derive(Default, Debug, Clone)]
pub struct Scroll {
    pub payload: UnsealedTransaction,
    pub stages: Stages,
    pub simulated_outcome: Option<Outcome>,
    pub live_outcome: Option<Outcome>,
}

impl Scroll {
    /// Loads the target account's database information into a [`CacheDB`].
    #[tracing::instrument(skip(self, forker))]
    fn load(
        &self,
        forker: &Forker,
        block: Option<u64>,
    ) -> anyhow::Result<CacheDB<EmptyDB>, anyhow::Error> {
        let target = self.payload.target.clone();
        let artifacts_path = self.payload.artifact.clone().to_str().unwrap().to_string();
        let ingest_payload = IngestPayload {
            target,
            artifacts_path,
        };
        let db = forker.load_cached_db(ingest_payload, block)?;

        Ok(db)
    }

    /// Loads the target account's database information before the transaction
    /// is executed.
    #[tracing::instrument(skip(self, forker))]
    fn load_before(&mut self, forker: &Forker, block: Option<u64>) -> anyhow::Result<()> {
        let db = self.load(forker, block)?;

        self.stages.before = Some(db);

        Ok(())
    }

    /// Loads the target account's database information after the transaction is
    /// executed.
    #[tracing::instrument(skip(self, forker))]
    pub fn load_after(&mut self, forker: &Forker, block: Option<u64>) -> anyhow::Result<()> {
        let db = self.load(forker, block)?;

        self.stages.after = Some(db);

        Ok(())
    }

    #[tracing::instrument(skip(self, forker))]
    pub async fn simulate(&mut self, forker: &Forker, block: Option<u64>) -> anyhow::Result<()> {
        // load the before stage if it hasn't been loaded yet.
        if self.stages.before.is_none() {
            self.load_before(forker, block)?;
        }

        tracing::trace!(
            "Loading storage into Arbiter: {:?}",
            self.stages.before.clone().unwrap()
        );
        // Loads the before stage into Arbiter and gets an Arbiter client.
        let db = self.stages.before.clone().unwrap();
        let environment = forker.load_env(db);
        let client = RevmMiddleware::new(&environment, Some("simulate"))?;
        let payload: TypedTransaction = self.payload.clone().try_into()?;

        // let client_cloned = client.clone();
        // let tx_handle: JoinHandle<anyhow::Result<Option<TransactionReceipt>,
        // anyhow::Error>> = tokio::spawn(async move {
        // Executes the transaction on the Arbiter client.
        // tracing::debug!("Sending simulation payload: {:?}", payload_copy);
        //
        // let tx = client_cloned
        // .send_transaction(payload_copy, None)
        // .await?
        // .await?;
        //
        // Ok(tx)
        // });
        //
        // let tx = tx_handle.await??;

        // Spawn a new thread to execute in the simulated environment
        // without it getting dropped.
        // let client_cloned = client.clone();
        // let payload_copy: TypedTransaction = self.payload.clone().try_into()?;
        // let handle = std::thread::spawn(move || {
        // Use a tokio runtime to block_on while the tx is processing.
        // let rt = tokio::runtime::Runtime::new()?;
        //
        // rt.block_on(async move {
        // let tx = client_cloned
        // .send_transaction(payload_copy, None)
        // .await?
        // .await?;
        // Ok::<_, anyhow::Error>(tx)
        // })
        // });
        // let sim_result = handle.join();
        //
        // match sim_result {
        // Ok(sim_result) => {
        // tracing::trace!("Sim result: {:?}", sim_result);
        // match sim_result {
        // Ok(sim_result) => {
        // tracing::trace!("Sim result ok: {:?}", sim_result);
        // }
        // Err(e) => {
        // tracing::trace!("Sim result error: {:?}", e);
        // }
        // }
        // }
        // Err(e) => {
        // tracing::trace!("Sim result error: {:?}", e);
        // }
        // }
        // let tx = sim_result.unwrap().unwrap();

        // Executes the transaction on the Arbiter client.

        tracing::debug!("Sending simulation payload: {:?}", payload);
        let tx = client
            .clone()
            .send_transaction(payload, None)
            .await?
            .await?;

        tracing::debug!("Simulated transaction: {:?}", tx);
        match tx {
            Some(tx) => {
                tracing::debug!("Simulated receipt: {:?}", tx.clone());
                self.simulated_outcome = Some(Outcome {
                    tx_hash: tx.clone().transaction_hash,
                    receipt: tx.clone(),
                });
            }
            None => {
                tracing::debug!("Simulated transaction failed");
            }
        }

        environment.stop()?;

        Ok(())
    }

    #[tracing::instrument(skip(self, forker))]
    pub async fn execute(
        &mut self,
        forker: &Forker,
        block: Option<u64>,
    ) -> anyhow::Result<TransactionReceipt, anyhow::Error> {
        // Return if the transaction has already been executed.
        if self.live_outcome.is_some() {
            return Err(anyhow::anyhow!("Transaction has already been executed."));
        }

        // Return if the transaction has not been simulated yet.
        if self.simulated_outcome.is_none() {
            return Err(anyhow::anyhow!("Transaction has not been simulated yet."));
        }

        let block: Option<BlockId> = match block {
            Some(block) => Some(BlockId::Number(block.into())),
            None => None,
        };

        // Executes the transaction on the live client.
        let payload: TypedTransaction = self.payload.clone().try_into()?;
        let client = forker.client.clone().unwrap();
        let tx = client.send_transaction(payload, block).await?.await?;
        tracing::debug!("Executed transaction: {:?}", tx);

        let res = match tx {
            Some(tx) => {
                tracing::debug!("Executed receipt: {:?}", tx.clone());
                self.live_outcome = Some(Outcome {
                    tx_hash: tx.clone().transaction_hash,
                    receipt: tx.clone(),
                });

                Ok(tx.clone())
            }
            None => {
                tracing::debug!("Executed transaction failed");
                Err(anyhow::anyhow!("Executed transaction failed"))
            }
        };

        res
    }
}

#[derive(Default, Debug, Clone)]
pub struct UnsealedTransaction {
    pub artifact: PathBuf,
    pub target: Address,
    pub value: Option<U256>,
    pub method: Option<String>,
    pub arguments: Vec<Token>,
}

impl TryFrom<UnsealedTransaction> for TypedTransaction {
    type Error = anyhow::Error;

    fn try_from(payload: UnsealedTransaction) -> anyhow::Result<Self, Self::Error> {
        // todo: maybe use this?
        // let mut req = TransactionRequest::new()
        // .to(payload.target)
        // .value(payload.value.into());

        let mut req = Eip1559TransactionRequest {
            from: None,
            to: Some(payload.target.into()),
            value: payload.value,
            gas: None,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            nonce: None,
            chain_id: None,
            access_list: vec![].into(),
            data: None,
        };

        if let Some(method) = payload.method {
            let path = payload.artifact.clone().to_str().unwrap().to_string();
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let contract_abi: ContractAbi = serde_json::from_reader(reader).unwrap();
            let abi = contract_abi.abi;
            let instance: BaseContract = abi.into();

            // todo: fix arg encoding so we can handle zero arg methods
            let data = instance.encode(method.as_str(), ()).unwrap();

            req.data = Some(data);
        } else {
            return Err(anyhow::anyhow!("No method specified in payload."));
        }

        let tx = TypedTransaction::Eip1559(req.into());

        Ok(tx)
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContractAbi {
    pub abi: ethers::abi::Abi,
}

impl UnsealedTransaction {
    /// Creates a new UnsealedTransaction that can be built into a Scroll.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn target(mut self, target: Address) -> Self {
        self.target = target;
        self
    }

    pub fn artifact(mut self, artifact: PathBuf) -> Self {
        self.artifact = artifact;
        self
    }

    /// Sets the value of the transaction.
    pub fn value(mut self, value: U256) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the method of the transaction.
    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    /// Sets the arguments of the transaction.
    pub fn arguments(mut self, arguments: Vec<Token>) -> Self {
        self.arguments = arguments;
        self
    }

    /// Adds an argument to the transaction.
    pub fn arg(mut self, arg: Token) -> Self {
        self.arguments.push(arg);
        self
    }

    /// Builds the transaction into a Scroll.
    pub fn seal(self) -> Scroll {
        Scroll {
            payload: self,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::utils::Anvil;
    use simulation::bindings::counter::Counter;

    use super::*;
    use crate::mvp::api::{forking::get_counter_path, tests::TEST_SUBSCRIBER};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_scroll() -> anyhow::Result<(), anyhow::Error> {
        // Global tracing subscriber
        let _ = *TEST_SUBSCRIBER;

        // Start anvil in the background.
        let anvil = Anvil::default()
            .arg("--gas-limit")
            .arg("20000000")
            .chain_id(31337_u64)
            .spawn();

        // Get the dev wallet.
        let wallet: LocalWallet = anvil.keys().get(0).unwrap().clone().into();

        // Create a forker.
        let forker = Forker::connect(Some(anvil.endpoint()), Some(wallet)).await?;

        // Get the block number.
        let anvil_client = forker.client.clone().unwrap();
        let block_number = anvil_client.get_block_number().await?;
        tracing::info!("Block number: {:?}", block_number);
        assert_eq!(block_number, 0.into());

        // Deploy the counter contract.
        let counter = Counter::deploy(anvil_client.clone(), ())?.send().await?;
        let counter_address = counter.address();

        // Build the unsealed "increment" transaction to get the Scroll.
        let target = counter_address;
        let value = U256::zero();
        let method = "increment";
        let arguments = vec![];
        let mut scroll = UnsealedTransaction::new()
            .target(target)
            .artifact(get_counter_path().unwrap())
            .value(value)
            .method(method)
            .arguments(arguments)
            .seal();

        // Update the forker with the desired block.
        // todo: move this logic into simulate?
        let block_number = 1_u64;
        let forker = forker.with_block_number(block_number);

        // Simulate the transaction.
        scroll.simulate(&forker, None).await?;

        // Execute the transaction.
        scroll.execute(&forker, None).await?;

        // Get the current block.
        let block_number = anvil_client.get_block_number().await?;

        // Update the forker.
        let forker = forker.with_block_number(block_number.as_u64());

        // Load the after db.
        scroll.load_after(&forker, None)?;

        // Log both the before and after dbs and the outcomes!
        tracing::debug!("Before: {:?}", scroll.stages.before);
        tracing::debug!("After: {:?}", scroll.stages.after);
        tracing::debug!("Simulated outcome: {:?}", scroll.simulated_outcome);

        // it works!

        Ok(())
    }
}
