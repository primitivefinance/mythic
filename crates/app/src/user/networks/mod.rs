//! All network related preferences and settings.

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainPacket {
    pub chain_id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RPCList {
    pub chains: HashMap<String, ChainPacket>,
}

impl RPCList {
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    pub fn add(&mut self, chain: ChainPacket) {
        self.chains.insert(chain.name.clone(), chain);
    }

    pub fn get(&self, name: &str) -> Option<&ChainPacket> {
        self.chains.get(name)
    }

    pub fn remove(&mut self, name: &str) {
        self.chains.remove(name);
    }

    pub fn list(&self) -> Vec<&ChainPacket> {
        self.chains.values().collect()
    }

    pub fn clear(&mut self) {
        self.chains.clear();
    }

    pub fn load_from_file(&mut self, path: &str) -> anyhow::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let mut value = String::new();
        file.read_to_string(&mut value)?;
        let value: RPCList = serde_json::from_str(&value)?;
        self.chains = value.chains;
        Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let value = serde_json::to_string_pretty(self)?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }
}
