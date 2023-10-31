use std::{collections::BTreeMap, fs::File, io::BufReader};

use anyhow::Result;
use ethers::types::U256;
use serde_json::{from_reader, Value};
pub use simulation::bindings::*;

#[allow(unused)]
pub mod reader;
#[cfg(test)]
mod tests;
#[allow(unused)]
pub mod visualize;

pub fn wad_to_float(wad: U256) -> f64 {
    wad.as_u128() as f64 / 10f64.powi(18)
}
