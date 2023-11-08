// TODO: Is it possible to just give every agent a reference to the client from
// the get go and use only that to construct them?
#![allow(non_snake_case, unused_imports)]

use std::sync::Arc;

use anyhow::Result;
use arbiter_core::{
    bindings::{
        arbiter_math::ArbiterMath, arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange,
    },
    data_collection::EventLogger,
    environment::{builder::EnvironmentBuilder, Environment},
    math::{OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::{errors::RevmMiddlewareError, RevmMiddleware},
};
use bindings::{
    atomic_arbitrage::{AtomicArbitrage, NotProfitable},
    g3m::G3M,
    rmm::RMM,
    sd5_9x_18_math::SD59x18Math,
};
use config::{Config, ConfigError};
use ethers::{
    abi::AbiDecode,
    types::{Address, TransactionReceipt, I256, U256},
    utils::{format_ether, format_units, parse_ether},
};
use serde::{Deserialize, Serialize};
use settings::SimulationConfig;
use tracing::{debug, error, info, trace, warn};

#[allow(unused)]
mod agents;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod bindings;
#[allow(unused)]
mod math;
#[allow(unused)]
pub mod settings;
#[allow(unused)]
pub mod simulations;
pub mod strategy;
#[cfg(test)]
pub(crate) mod tests;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);
