//! Utilities for developers.

use datatypes::{portfolio::weight::Weight, weight};

pub const COIN_X: &str = r#"{
    "symbol": "X",
    "name": "X",
    "decimals": 18,
    "tags": [],
    "chain_id": 31337,
    "address": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    "logo_uri": ""
}"#;

pub const COIN_Y: &str = r#"{
    "symbol": "Y",
    "name": "Y",
    "decimals": 18,
    "tags": [],
    "chain_id": 31337,
    "address": "0x5fbdb2315678afecb367f032d93f642f64180aa4",
    "logo_uri": ""
}"#;

pub const INITIAL_X_WEIGHT: Weight = weight!(0.01);
pub const INITIAL_X_PRICE: f64 = 1.0;
pub const INITIAL_X_BALANCE: f64 = 1.0;
pub const INITIAL_Y_WEIGHT: Weight = weight!(0.99);
pub const INITIAL_Y_PRICE: f64 = 1.0;
pub const INITIAL_Y_BALANCE: f64 = 99.0;
