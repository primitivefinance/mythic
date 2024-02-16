//! A Coin List implements the Token List standard, so that these
//! coin lists can be serialized and deserialized directly from the
//! Token List JSON format.

use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Div, Sub},
};

use serde::{Deserialize, Serialize};

use super::coin::Coin;

/// Custom tags for labeling the coin.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoinTags {
    pub name: String,
    pub description: String,
}

/// Version of the coin list.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoinListVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

/// Token List standard implementation for a coin list.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoinList {
    pub name: String,
    pub logo_uri: String,
    pub keywords: Vec<String>,
    pub timestamp: String,
    pub tags: Vec<CoinTags>,
    pub tokens: Vec<Coin>,
    pub token_map: BTreeMap<String, Coin>,
    pub version: CoinListVersion,
}

impl CoinList {
    /// Token list standard keeps a map of the tokens with keys in the format of
    /// {chain_id}_{address}.
    pub fn get_mapping_key(&self, coin: &Coin) -> String {
        format!("{}_{:?}", coin.chain_id, coin.address)
    }

    /// Constructs the token map from the existing coins in the list using the
    /// mapping keys.
    pub fn build_map(&mut self) {
        let mut token_map = BTreeMap::new();
        for token in self.tokens.iter() {
            let key = self.get_mapping_key(token);
            token_map.insert(key, token.clone());
        }
        self.token_map = token_map;
    }
}

impl Add for CoinList {
    type Output = Self;

    /// Adds two coin lists together.
    fn add(self, rhs: Self) -> Self::Output {
        let mut tokens = self.tokens;
        tokens.extend(rhs.tokens);
        let mut tags = self.tags;
        tags.extend(rhs.tags);
        let mut keywords = self.keywords;
        keywords.extend(rhs.keywords);
        let mut result = Self {
            name: self.name,
            logo_uri: self.logo_uri,
            keywords,
            timestamp: self.timestamp,
            tags,
            tokens,
            token_map: BTreeMap::new(),
            version: self.version,
        };
        result.build_map();
        result
    }
}

impl Sub for CoinList {
    type Output = Self;

    /// Removes any tokens that exist in both lists.
    fn sub(self, rhs: Self) -> Self::Output {
        let mut tokens = self.tokens;
        tokens.retain(|token| !rhs.tokens.contains(token));
        let mut tags = self.tags;
        tags.retain(|tag| !rhs.tags.contains(tag));
        let mut keywords = self.keywords;
        keywords.retain(|keyword| !rhs.keywords.contains(keyword));

        let mut result = Self {
            name: self.name,
            logo_uri: self.logo_uri,
            keywords,
            timestamp: self.timestamp,
            tags,
            tokens,
            token_map: BTreeMap::new(),
            version: self.version,
        };
        result.build_map();
        result
    }
}

impl Add<Coin> for CoinList {
    type Output = Self;

    fn add(mut self, rhs: Coin) -> Self::Output {
        self.tokens.push(rhs);
        self
    }
}

impl Sub<Coin> for CoinList {
    type Output = Self;

    fn sub(mut self, rhs: Coin) -> Self::Output {
        self.tokens.retain(|token| *token != rhs);
        self
    }
}

impl AddAssign<Coin> for CoinList {
    fn add_assign(&mut self, rhs: Coin) {
        self.tokens.push(rhs);
    }
}

impl Div for CoinList {
    type Output = Self;

    /// Cancels out any tokens that exist in both lists.
    fn div(self, rhs: Self) -> Self::Output {
        let mut tokens = self.tokens;
        tokens.retain(|token| !rhs.tokens.contains(token));
        let mut tags = self.tags;
        tags.retain(|tag| !rhs.tags.contains(tag));
        let mut keywords = self.keywords;
        keywords.retain(|keyword| !rhs.keywords.contains(keyword));

        let mut result = Self {
            name: self.name,
            logo_uri: self.logo_uri,
            keywords,
            timestamp: self.timestamp,
            tags,
            tokens,
            token_map: BTreeMap::new(),
            version: self.version,
        };
        result.build_map();
        result
    }
}
