//! Most basic types for managing portfolios.

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

/// Most simple representation of all positions in a portfolio: balance and
/// cost.
/// - balance: Amount of asset held.
/// - cost: Cost of asset held.
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub balance: f64,
    pub cost: f64,
}

impl Position {
    pub fn new(_asset: &str, balance: f64, cost: f64) -> Position {
        Position { balance, cost }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Position {{ balance: {}, cost: {} }}",
            self.balance, self.cost
        )
    }
}

/// A portfolio is a collection of positions, with identifier keys.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    pub positions: HashMap<String, Position>,
}

/// Portfolios add, remove, or edit positions.
impl Portfolio {
    /// Adds a position to the portfolio.
    /// - asset: Asset identifier.
    /// - balance: Amount of asset held.
    /// - cost: Cost of asset held.
    pub fn add_position(&mut self, asset: &str, balance: f64, cost: f64) {
        let position = Position { balance, cost };
        self.positions.insert(asset.to_string(), position);
    }

    /// Removes a position from the portfolio.
    /// - asset: Asset identifier.
    pub fn remove_position(&mut self, asset: &str) {
        self.positions.remove(asset);
    }

    /// Edits a position in the portfolio.
    /// - asset: Asset identifier.
    /// - balance: Amount of asset held.
    /// - cost: Cost of asset held.
    pub fn edit_position(&mut self, asset: &str, balance: f64, cost: f64) {
        self.add_position(asset, balance, cost);
    }

    /// Returns the balance of a position in the portfolio.
    /// - asset: Asset identifier.
    pub fn get_balance(&self, asset: &str) -> f64 {
        self.positions
            .get(asset)
            .map(|position| position.balance)
            .unwrap_or(0.0)
    }

    /// Returns the cost of a position in the portfolio.
    /// - asset: Asset identifier.
    pub fn get_cost(&self, asset: &str) -> f64 {
        self.positions
            .get(asset)
            .map(|position| position.cost)
            .unwrap_or(0.0)
    }

    /// Returns the total portfolio value.
    pub fn get_total_value(&self) -> f64 {
        self.positions
            .iter()
            .map(|(asset, position)| {
                let balance = position.balance;
                let cost = position.cost;
                let value = balance * cost;
                (asset, value)
            })
            .map(|(_, value)| value)
            .sum()
    }
}

impl Display for Portfolio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Portfolio {{ positions: {:?} }}", self.positions)
    }
}

/// A manager controls multiple portfolios.
#[derive(Debug, Clone, Default)]
pub struct Manager {
    pub portfolios: HashMap<String, Portfolio>,
}

impl Manager {
    /// Makes a new portfolio
    pub fn construct_portfolio(&mut self, name: &str) {
        let portfolio = Portfolio::default();
        self.portfolios.insert(name.to_string(), portfolio);
    }

    /// Edits an existing portfolio
    pub fn edit_portfolio(&mut self, name: &str) {
        self.construct_portfolio(name);
    }
}

impl Display for Manager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Manager {{ portfolios: {:?} }}", self.portfolios)
    }
}
