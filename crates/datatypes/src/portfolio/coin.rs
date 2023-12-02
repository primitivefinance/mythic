//! The most primitive type of a portfolio item is a coin.
//! These coins implement the Token List standard and are serializable for easy
//! storage.
//! Why is it not called token? Because it is not a token. It is a coin. A token
//! could be many things. A coin is a coin.

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

/// Implements the Token List standard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Coin {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub tags: Vec<String>,
    pub chain_id: u64,
    pub address: Address,
    pub logo_uri: String,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
            symbol: "Symbol".to_string(),
            name: "Name".to_string(),
            decimals: 18,
            tags: vec![],
            chain_id: 1,
            address: Address::default(),
            logo_uri: "".to_string(),
        }
    }
}

impl Coin {
    pub fn new(symbol: String, name: String, decimals: u8) -> Self {
        Self {
            symbol,
            name,
            decimals,
            ..Default::default()
        }
    }
}

impl std::ops::Add for Coin {
    type Output = super::coin_list::CoinList;

    fn add(self, rhs: Self) -> Self::Output {
        let tokens = vec![self, rhs];
        let mut coin_list = super::coin_list::CoinList::default();
        coin_list.tokens = tokens;
        coin_list
    }
}
