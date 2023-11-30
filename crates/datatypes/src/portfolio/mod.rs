//! Portfolio related data types and models.
pub mod coin;
pub mod coin_list;
pub mod nwd;
pub mod position;
pub mod weight;

use std::ops::Mul;

use position::{Position, Positions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
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
}

#[macro_export]
macro_rules! portfolio_weights {
    ($portfolio:expr) => {
        $portfolio
            .positions
            .0
            .iter()
            .map(|position| position.weight.unwrap_or(Weight::default()).value)
            .collect::<Vec<f64>>()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::portfolio::{coin::Coin, position::Position, weight::Weight};

    #[test]
    pub fn test_portfolio_weights_macro() {
        let position = Position::new(
            Coin::new("test".to_string(), "test".to_string(), 18),
            Some(0.5),
            Some(0.5),
            Some(Weight::new(0.5).unwrap()),
            Some(0.5),
        );
        let portfolio = Portfolio::new(
            "test".to_string(),
            "test".to_string(),
            vec![position.clone(), position.clone()],
        );

        let weight = portfolio.positions.0[0].clone().weight.unwrap_or_default();
        println!("{weight:?}");
        assert_eq!(portfolio_weights!(portfolio), vec![0.5, 0.5]);
    }
}
