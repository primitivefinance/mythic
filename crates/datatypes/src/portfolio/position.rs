use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use anyhow::Result;
/// ! A position is an individual portion of a portfolio.
use serde::{Deserialize, Serialize};
use simulation::agents::token_admin::TokenData;

use super::{
    coin::Coin,
    nwd::NWD,
    weight::{Weight, WeightError},
};

/// A data type for a position in a portfolio.
///
/// Examples:
///
/// ```
/// use datatypes::portfolio::{
///     coin::Coin,
///     position::{Position, Positions},
///     weight::Weight,
/// };
///
/// let mut positions = Positions::new(vec![
///     Position::new(
///         Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
///         Some(100.0),
///         Some(1.0),
///         Some(Weight::new(0.5).unwrap()),
///         Some(0.5),
///     ),
///     Position::new(
///         Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
///         Some(100.0),
///         Some(1.0),
///         Some(Weight::new(0.5).unwrap()),
///         Some(0.5),
///     ),
/// ]);
///
/// // Add a position
/// positions += Position::new(
///     Coin::new("Coin".to_string(), "COIN".to_string(), 8),
///     Some(100.0),
///     Some(1.0),
///     Some(Weight::new(0.5).unwrap()),
///     Some(0.5),
/// );
///
/// assert_eq!(positions.len(), 3);
/// assert_eq!(
///     positions,
///     Positions(vec![
///         Position::new(
///             Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.25).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.25).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Coin".to_string(), "COIN".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.5).unwrap()),
///             Some(0.5),
///         ),
///     ])
/// );
///
/// // Remove a position
/// positions -= positions.0[0].clone();
/// assert_eq!(positions.len(), 3);
/// assert_eq!(
///     positions,
///     Positions(vec![
///         Position::new(
///             Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.0).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.25 + 0.125).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Coin".to_string(), "COIN".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.5 + 0.125).unwrap()),
///             Some(0.5),
///         ),
///     ])
/// );
///
/// // Adjust an existing position
/// positions
///     .adjust(positions.0[0].weight.unwrap_or_default().id, 0.25)
///     .unwrap();
///
/// assert_eq!(positions.len(), 3);
/// assert_eq!(
///     positions,
///     Positions(vec![
///         Position::new(
///             Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.0 + 0.25).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.25 + 0.125 - 0.125).unwrap()),
///             Some(0.5),
///         ),
///         Position::new(
///             Coin::new("Coin".to_string(), "COIN".to_string(), 8),
///             Some(100.0),
///             Some(1.0),
///             Some(Weight::new(0.5 + 0.125 - 0.125).unwrap()),
///             Some(0.5),
///         ),
///     ])
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct Position {
    /// Underlying asset of the position.
    pub asset: Coin,
    /// Average cost basis of the position.
    pub cost: Option<f64>,
    /// Current actual balance of coins the position holds.
    pub balance: Option<f64>,
    /// Target weight for the position, not actual weight.
    pub weight: Option<Weight>,
    /// Target volatility for the position, no the actual volatility.
    pub volatility: Option<f64>,
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.asset.hash(state);
    }
}

impl Position {
    pub fn new(
        asset: Coin,
        cost: Option<f64>,
        balance: Option<f64>,
        weight: Option<Weight>,
        volatility: Option<f64>,
    ) -> Self {
        Self {
            asset,
            cost,
            balance,
            weight,
            volatility,
        }
    }
}

/// TokenData is used in simulations for the token admin agent.
/// todo: find a better from for this data structure.
impl From<Position> for TokenData {
    fn from(position: Position) -> Self {
        Self {
            name: position.asset.name,
            symbol: position.asset.symbol,
            decimals: position.asset.decimals,
        }
    }
}

/// Data structure for a group of positions, that is ruled by the underlying
/// Weight data type's arithmetic rules.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct Positions(pub Vec<Position>);

impl From<Vec<Position>> for Positions {
    fn from(positions: Vec<Position>) -> Self {
        Self(positions)
    }
}

impl Positions {
    pub fn new(positions: Vec<Position>) -> Self {
        Self(positions)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_nwd(&self) -> NWD {
        let weights = self
            .0
            .iter()
            .map(|position| position.weight.unwrap_or_default())
            .collect::<Vec<_>>();
        NWD(weights)
    }

    /// Adjusts an individual weight in the positions.
    pub fn adjust(&mut self, id: uuid::Uuid, delta: f64) -> Result<(), PositionError> {
        // Find the position being changed.
        let position = self
            .0
            .iter_mut()
            .find(|position| position.weight.unwrap_or_default().id == id)
            .cloned()
            .ok_or(PositionError::PositionDoesNotExist)?;

        // Clone the weight and replace it's value with the delta.
        let mut weight = position.weight.unwrap_or_default().clone();
        weight.set_value(delta)?;

        // Add the weight to the NWD.
        let nwd = self.as_nwd().clone() + weight;

        // Adjust the positions to the new weights.
        let positions = &mut self.0;
        for (i, position) in positions.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }

        Ok(())
    }
}

impl Add<Position> for Positions {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        // First, get the current NWD and add the new weight to it.
        let nwd = self.as_nwd().clone() + rhs.weight.unwrap_or_default();

        // Adjust the positions to the new weights.
        let mut positions = self.0;
        for (i, position) in positions.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }

        // Add the position to the list.
        positions.push(rhs);

        Self(positions)
    }
}

impl Sub<Position> for Positions {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        // First, get the current NWD and subtract the weight from it.
        let nwd = self.as_nwd().clone() - rhs.weight.unwrap_or_default();

        // Adjust the positions to the new weights.
        let mut positions = self.0;
        for (i, position) in positions.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }

        // Return the new positions.
        Self(positions)
    }
}

impl AddAssign<Position> for Positions {
    fn add_assign(&mut self, rhs: Position) {
        // First, get the current NWD and add the new weight to it.
        let nwd = self.as_nwd().clone() + rhs.weight.unwrap_or_default();

        // Adjust the positions to the new weights.
        for (i, position) in self.0.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }

        // Add the position to the list.
        self.0.push(rhs);
    }
}

impl SubAssign<Position> for Positions {
    fn sub_assign(&mut self, rhs: Position) {
        // First, get the current NWD and subtract the weight from it.
        let nwd = self.as_nwd().clone() - rhs.weight.unwrap_or_default();

        // Adjust the positions to the new weights.
        for (i, position) in self.0.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }
    }
}

impl From<WeightError> for PositionError {
    fn from(err: WeightError) -> Self {
        Self::WeightError(err)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum PositionError {
    /// The position does not exist.
    PositionDoesNotExist,
    /// The position already exists.
    PositionAlreadyExists,
    /// Got a weight error.
    WeightError(WeightError),
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionError::PositionDoesNotExist => {
                write!(f, "The position does not exist.")
            }
            PositionError::PositionAlreadyExists => {
                write!(f, "The position already exists.")
            }
            PositionError::WeightError(err) => {
                write!(f, "Got a weight error: {}", err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_add() {
        let mut positions = Positions::new(vec![Position::new(
            Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
            Some(100.0),
            Some(1.0),
            Some(Weight::new(1.0).unwrap()),
            Some(0.5),
        )]);

        positions += Position::new(
            Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
            Some(100.0),
            Some(1.0),
            Some(Weight::new(0.5).unwrap()),
            Some(0.5),
        );

        assert_eq!(
            positions,
            Positions(vec![
                Position::new(
                    Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
                    Some(100.0),
                    Some(1.0),
                    Some(Weight::new(0.5).unwrap()),
                    Some(0.5),
                ),
                Position::new(
                    Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
                    Some(100.0),
                    Some(1.0),
                    Some(Weight::new(0.5).unwrap()),
                    Some(0.5),
                ),
            ])
        );
    }

    #[test]
    fn test_positions_sub() {
        let mut positions = Positions::new(vec![
            Position::new(
                Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
                Some(100.0),
                Some(1.0),
                Some(Weight::new(0.5).unwrap()),
                Some(0.5),
            ),
            Position::new(
                Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
                Some(100.0),
                Some(1.0),
                Some(Weight::new(0.5).unwrap()),
                Some(0.5),
            ),
        ]);

        positions -= positions.0[0].clone();

        assert_eq!(
            positions,
            Positions(vec![
                Position::new(
                    Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
                    Some(100.0),
                    Some(1.0),
                    Some(Weight::new(0.0).unwrap()),
                    Some(0.5),
                ),
                Position::new(
                    Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
                    Some(100.0),
                    Some(1.0),
                    Some(Weight::new(1.0).unwrap()),
                    Some(0.5),
                ),
            ])
        );
    }
}
