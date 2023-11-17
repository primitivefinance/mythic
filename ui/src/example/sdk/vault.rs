//! Rust api over the vault smart contract.

#![allow(unused_variables)]
use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use ethers::prelude::*;
use simulation::bindings::counter::Counter;

#[derive(Debug, Clone)]
pub struct Vault {
    pub valid: bool,
    pub address: Address,
}

impl Vault {
    pub async fn deploy<E>(client: Arc<RevmMiddleware>) -> Result<Self, E> {
        let instance = Counter::deploy(client, ()).unwrap().send().await.unwrap();
        Ok(Self {
            valid: true,
            address: instance.address(),
        })
    }

    pub async fn instance<E>(
        &self,
        client: Arc<RevmMiddleware>,
    ) -> Result<Counter<RevmMiddleware>, E> {
        let instance = Counter::new(self.address, client);
        Ok(instance)
    }
}
