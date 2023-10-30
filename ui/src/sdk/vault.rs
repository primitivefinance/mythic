//! Rust api over the vault smart contract.

#![allow(unused_variables)]
use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use simulation::bindings::counter::Counter;

#[derive(Debug, Clone)]
pub struct Vault {
    pub valid: bool,
}

impl Vault {
    pub async fn deploy<E>(client: Arc<RevmMiddleware>) -> Result<Self, E> {
        let instance = Counter::deploy(client, ()).unwrap().send().await.unwrap();
        Ok(Self { valid: true })
    }
}
