use std::{collections::BTreeMap, fs::File, io::BufReader};

use anyhow::Result;
use serde_json::{from_reader, Value};

use simulation::bindings::*;

#[allow(unused)]
pub mod reader;