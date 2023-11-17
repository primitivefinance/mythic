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

use ethers::abi::{Token, Tokenizable};
use revm::db::{CacheDB, EmptyDB};

use super::*;

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
    pub outcome: Option<Outcome>,
}

impl Scroll {
    async fn load_database(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct UnsealedTransaction {
    pub target: Address,
    pub value: Option<U256>,
    pub method: Option<String>,
    pub arguments: Vec<Token>,
}

impl UnsealedTransaction {
    /// Creates a new UnsealedTransaction that can be built into a Scroll.
    pub fn new(target: Address) -> Self {
        Self {
            target,
            ..Default::default()
        }
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

    /// Builds the transaction into a Scroll.
    pub fn seal(self) -> Scroll {
        Scroll {
            payload: self,
            ..Default::default()
        }
    }
}
