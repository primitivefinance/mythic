use std::{collections::HashMap, fs::File};

use super::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct StaticCoin {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub tags: Vec<String>,
    pub chain_id: u64,
    // todo: does this work for token list standard?
    pub address: String,
    pub logo_uri: String,
}

impl StaticCoin {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoinTags {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoinListVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoinList {
    pub name: String,
    pub logo_uri: String,
    pub keywords: Vec<String>,
    pub timestamp: String,
    pub tags: Vec<CoinTags>,
    pub tokens: Vec<StaticCoin>,
    pub token_map: HashMap<String, StaticCoin>,
    pub version: CoinListVersion,
}

impl CoinList {
    pub fn add_token(&mut self, token: StaticCoin) {
        self.tokens.push(token);
    }

    pub fn get_mapping_key(&self, token: &StaticCoin) -> String {
        format!("{}_{}", token.chain_id, token.address)
    }

    pub fn build_token_map(&mut self) {
        let mut token_map = HashMap::new();
        for token in self.tokens.iter() {
            let key = self.get_mapping_key(token);
            token_map.insert(key, token.clone());
        }
        self.token_map = token_map;
    }
}

const COIN_LIST_EXTENSION: &'static str = "json";
const COIN_LIST_SUFFIX: &'static str = "coinlist";

impl Saveable for CoinList {
    fn prefix(&self) -> Option<String> {
        println!("prefix: {:?} {}", self.name, self.name.clone().is_empty());
        match !self.name.clone().is_empty() {
            true => Some(self.name.clone()),
            false => None,
        }
    }

    fn create_new(name: Option<String>) -> Result<Self> {
        let path = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };

        let file = File::create(path)?;

        let mut coinlist = Self::default();
        match name {
            Some(name) => coinlist.name = name,
            None => {}
        }

        serde_json::to_writer_pretty(file, &coinlist)?;
        Ok(coinlist)
    }

    const EXTENSION: &'static str = COIN_LIST_EXTENSION;
    const SUFFIX: &'static str = COIN_LIST_SUFFIX;
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_coinlist_create() {
        let result = CoinList::create_new(Some("test".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().file_path()).exists());
    }

    #[test]
    fn test_coinlist_load() {
        let name = Some("test".to_string());
        let file_path = CoinList::file_path_with_name(name.unwrap());
        let result = CoinList::load(Some(file_path));
        println!("Loaded coinlist: {:?}", result);
        assert!(result.is_ok());
        assert_eq!("test".to_string(), result.unwrap().name);
    }
}
