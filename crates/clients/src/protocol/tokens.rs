use super::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: usize,
    pub address: Address,
}

impl Token {
    pub fn new(name: String, symbol: String, decimals: usize, address: Address) -> Result<Self> {
        Ok(Self {
            name,
            symbol,
            decimals,
            address,
        })
    }
}
