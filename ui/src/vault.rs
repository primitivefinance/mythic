//! Rust api over the vault smart contract.

#![allow(unused_variables)]
use arbiter_core::middleware::{errors::RevmMiddlewareError, RevmMiddleware};
use simulation::bindings::counter::Counter;

use std::sync::Arc;

use crate::example::Error;

#[derive(Debug, Clone)]
pub struct Vault {
    pub valid: bool,
}

impl Vault {
    pub async fn deploy(client: Arc<RevmMiddleware>) -> Result<Self, Error> {
        let instance = Counter::deploy(client, ()).unwrap().send().await.unwrap();
        Ok(Self { valid: true })
    }
}

impl From<RevmMiddlewareError> for Error {
    fn from(_error: RevmMiddlewareError) -> Self {
        match _error {
            _ => Error::BlockSubscriptionError,
        }
    }
}
