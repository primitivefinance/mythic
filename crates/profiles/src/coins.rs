use std::{collections::HashMap, fs::File};

use super::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct StaticCoin {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub tags: Vec<String>,
    pub chain_id: u16,
    // todo: does this work for token list standard?
    pub address: String,
    pub logo_uri: String,
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

const COIN_LIST_EXTENSION: &'static str = "json";
const COIN_LIST_SUFFIX: &'static str = "coinlist";

impl Saveable for CoinList {
    fn prefix(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_new(name: Option<String>) -> Result<Self> {
        let name = name.unwrap_or_else(|| "default".to_string());
        let path =
            Self::config_dir().join(format!("{}.{}.{}", name, Self::SUFFIX, Self::EXTENSION));
        let file = File::create(path)?;

        let mut coinlist = Self::default();
        coinlist.name = name;

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
