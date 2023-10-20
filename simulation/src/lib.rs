use std::{ops::Div, sync::Arc};

use anyhow::Result;
use arbiter_core::{
    bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange},
    data_collection::EventLogger,
    environment::{builder::EnvironmentBuilder, Environment},
    math::{float_to_wad, OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::{errors::RevmMiddlewareError, RevmMiddleware},
};
use bindings::{
    atomic_arbitrage::{AtomicArbitrage, NotProfitable},
    g3m::G3M,
    sd5_9x_18_math::SD59x18Math,
};
use config::{Config, ConfigError};
use ethers::{
    abi::AbiDecode,
    types::{Address, I256, U256},
    utils::{format_ether, format_units, parse_ether},
};
use serde::{Deserialize, Serialize};
use settings::SimulationConfig;
use tracing::info;

#[allow(unused)]
mod agents;
pub mod bindings;
#[allow(unused)]
mod math;
#[allow(unused)]
mod settings;
#[allow(unused)]
pub mod simulations;
