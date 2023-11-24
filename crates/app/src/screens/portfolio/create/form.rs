//! Implements the form for creating a portfolio.

#[derive(Debug, Clone, Default)]
pub struct Form {
    name: String,
    ticker: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Clone, Default)]
pub struct Asset {
    ticker: String,
    price: f64,
    balance: f64,
    selected: bool,
}
