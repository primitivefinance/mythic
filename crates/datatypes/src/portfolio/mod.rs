//! Portfolio related data types and models.
pub mod coin;
pub mod coin_list;
pub mod nwd;
pub mod position;
pub mod weight;

use std::ops::Mul;

use position::{Position, Positions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Portfolio {
    pub name: String,
    pub ticker: String,
    pub positions: Positions,
}

impl Portfolio {
    pub fn new(name: String, ticker: String, positions: Vec<Position>) -> Self {
        let positions = Positions::from(positions);
        Self {
            name,
            ticker,
            positions,
        }
    }

    /// Sum of all the products of the position's balance and price.
    #[tracing::instrument(skip(self), ret)]
    pub fn compute_total_portfolio_value(&self) -> f64 {
        self.positions
            .0
            .iter()
            .map(|position| {
                position
                    .balance
                    .unwrap_or(0.0)
                    .mul(position.cost.unwrap_or(0.0))
            })
            .sum()
    }
}
