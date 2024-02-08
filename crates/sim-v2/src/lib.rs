pub mod agents;

use std::{any::Any, collections::HashMap, fmt::Display, path::Path, sync::Arc};

use ::config::ConfigError;
use agents::{
    liquidity_provider::LiquidityProvider, price_changer::PriceChanger,
    protocol_manager::ProtocolManager, token_admin::TokenAdmin,
};
use anyhow::Result;
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::{environment::Environment, middleware::ArbiterMiddleware};
use arbiter_engine::{
    errors::ArbiterEngineError,
    machine::{
        Behavior, ControlFlow,
        ControlFlow::{Continue, Halt},
        CreateStateMachine, Engine, EventStream, StateMachine,
    },
    messager::{Message, Messager, To},
    world::World,
};
use arbiter_macros::Behaviors;
use clients::protocol::{ProtocolClient, ProtocolClientAddresses};
use ethers::{prelude::*, utils::parse_ether};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tracing::{debug, error, trace, warn};

pub const WAD: U256 = U256([1_000_000_000_000_000_000, 0, 0, 0]);
pub const MAX: U256 = U256::MAX;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<Address>,
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: Address,

    /// The amount to mint.
    pub mint_amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Behaviors)]
enum DfmmBehaviors {
    TokenAdmin(TokenAdmin),
    ProtocolManager(ProtocolManager),
    LiquidityProvider(LiquidityProvider),
    PriceChanger(PriceChanger),
}

pub fn run() -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    // Run the sims, returning snapshot dbs to the manager's `instances`.
    rt.block_on(async move {
        let mut world = World::new("sim");
        world.from_config::<DfmmBehaviors>("./crates/sim-v2/src/config.toml");
        world.run().await
    })?;

    Ok(())
}
