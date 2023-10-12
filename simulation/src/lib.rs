use std::{ops::Div, sync::Arc};

use anyhow::Result;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange,
    data_collection::EventLogger,
    environment::{builder::EnvironmentBuilder, Environment},
    math::{float_to_wad, OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::RevmMiddleware,
};
use bindings::g3m::G3M;
use ethers::{
    types::{Address, I256, U256},
    utils::{format_ether, format_units, parse_ether},
};
use settings::SimulationConfig;
use tracing::info;

use arbiter_core::bindings::arbiter_token::ArbiterToken;

use arbiter_core::middleware::errors::RevmMiddlewareError;
use bindings::{
    atomic_arbitrage::{AtomicArbitrage, NotProfitable},
    sd5_9x_18_math::SD59x18Math,
};
use ethers::abi::AbiDecode;

mod agents;
mod math;
mod settings;
pub mod simulations;
