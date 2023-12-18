//! Portfolio related data types and models.
pub mod coin;
pub mod coin_list;
pub mod nwd;
pub mod position;
pub mod weight;

use std::ops::{AddAssign, SubAssign};

use position::{Position, Positions};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{nwd::NWD, position::PositionError, weight::Weight};
use super::TokenData;

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

    pub fn adjust(&mut self, id: Uuid, delta: f64) -> Result<(), PositionError> {
        self.positions.adjust(id, delta)
    }

    pub fn aum(&self) -> f64 {
        self.positions.aum()
    }

    pub fn sync_prices(&mut self, prices: Vec<f64>) -> Result<(), PositionError> {
        self.positions.sync_prices(prices);

        Ok(())
    }

    pub fn weights(&self) -> Vec<f64> {
        self.positions.weights()
    }

    pub fn nwd(&self) -> NWD {
        self.positions.as_nwd()
    }
}

impl AddAssign<Position> for Portfolio {
    fn add_assign(&mut self, rhs: Position) {
        self.positions += rhs;
    }
}

impl AddAssign<Vec<Position>> for Portfolio {
    fn add_assign(&mut self, rhs: Vec<Position>) {
        self.positions += Positions::from(rhs);
    }
}

impl SubAssign<Position> for Portfolio {
    fn sub_assign(&mut self, rhs: Position) {
        self.positions -= rhs;
    }
}

impl From<Vec<Position>> for Portfolio {
    fn from(positions: Vec<Position>) -> Self {
        Portfolio { positions: Positions::from(positions), ..Default::default() }
    }
}

impl From<Positions> for Portfolio {
    fn from(positions: Positions) -> Self {
        Self {
            positions,
            ..Default::default()
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
