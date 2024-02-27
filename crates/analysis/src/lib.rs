use std::{collections::BTreeMap, fs::File, io::BufReader};

use anyhow::Result;
use reader::SimulationData;
use serde_json::{from_reader, Value};
use tracing::{debug, info};

pub mod reader;
pub mod unpacker;
pub mod visualize;

pub mod commands;

pub use reader::*;
pub use unpacker::*;
pub use visualize::*;
