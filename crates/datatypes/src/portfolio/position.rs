use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use anyhow::Result;
use chrono;
/// ! A position is an individual portion of a portfolio.
use serde::{Deserialize, Serialize};

use super::{
    coin::Coin,
    nwd::NWD,
    weight::{Weight, WeightError},
    TokenData,
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
    /// Information about the position.
    pub information: Option<Information>,
    /// Position's layer in the portfolio.
    pub layer: Option<PositionLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum PositionLayer {
    #[default]
    RawBalance,
    Liquidity,
    Staked,
    Collateral,
}

/// Carries information that can be used for synchronization, debugging, and
/// logging.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Information {
    /// The last time the prices were synchronized for a given position.
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    /// Configuration for considering the position's price stale.
    pub time_to_stale: u64,
}

impl Default for Information {
    fn default() -> Self {
        Self {
            last_sync: None,
            time_to_stale: 300 as u64,
        }
    }
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
            information: Some(Information::default()),
            layer: None,
        }
    }

    pub fn layer(mut self, layer: PositionLayer) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn is_stale(&self) -> bool {
        if let Some(information) = &self.information {
            if let Some(last_sync) = information.last_sync {
                let tts = chrono::Duration::seconds(information.time_to_stale as i64);
                return last_sync + tts < chrono::Utc::now();
            }
        }
        false
    }

    pub fn sync_price(&mut self, price: f64) {
        self.cost = Some(price);
        if let Some(ref mut information) = self.information {
            information.last_sync = Some(chrono::Utc::now());
        } else {
            self.information = Some(Information {
                last_sync: Some(chrono::Utc::now()),
                ..Default::default()
            });
        }
    }

    pub fn market_value(&self) -> f64 {
        self.cost.unwrap_or_default() * self.balance.unwrap_or_default()
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

    /// Given a desired weight change, adjusts the weights and balances of the
    /// positions in the normalized weight distribution.
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
        // Create an identical weight (id-wise) with the delta we want to apply in
        // absolute terms.
        weight.set_value(delta.abs())?;
        let is_negative = delta < 0.0;

        // Get the aum before the adjustment to compute the balance adjustments later.
        let aum = self.aum();

        // Add the weight to the NWD.
        let nwd = match is_negative {
            false => self.as_nwd().clone() + weight,
            true => self.as_nwd().clone() - weight,
        };

        // Adjust the positions to the new weights and balances.
        let positions = &mut self.0;
        for (i, position) in positions.iter_mut().enumerate() {
            let weight = nwd.0[i];
            position.weight = Some(weight);
            position.balance = Some(Self::compute_balance(
                position.cost.unwrap_or_default(),
                aum,
                weight.value,
            ));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn weights(&self) -> Vec<f64> {
        self.0
            .iter()
            .map(|position| position.weight.unwrap_or_default().value)
            .collect::<Vec<_>>()
    }

    pub fn as_nwd(&self) -> NWD {
        let weights = self
            .0
            .iter()
            .map(|position| position.weight.unwrap_or_default())
            .collect::<Vec<_>>();
        NWD(weights)
    }

    /// Synchronizes the costs of the positions with the given prices.
    pub fn sync_prices(&mut self, prices: Vec<f64>) {
        for (i, position) in self.0.iter_mut().enumerate() {
            position.sync_price(prices[i]);
        }
    }

    /// Assets under management in value terms.
    /// Sum of all the products of the position's balance and price.
    #[tracing::instrument(skip(self), ret)]
    pub fn aum(&self) -> f64 {
        self.0
            .iter()
            .map(|position| {
                position
                    .balance
                    .unwrap_or_default()
                    .mul(position.cost.unwrap_or_default())
            })
            .sum()
    }

    /// Computes the balance of a position given the weight, price, and AUM.
    pub fn compute_balance(price: f64, aum: f64, weight: f64) -> f64 {
        weight * aum / price
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

impl AddAssign<Positions> for Positions {
    fn add_assign(&mut self, rhs: Positions) {
        // First, get the current NWD and add the new weight to it.
        let nwd = self.as_nwd().clone() + rhs.as_nwd().clone();

        // Adjust the positions to the new weights.
        for (i, position) in self.0.iter_mut().enumerate() {
            position.weight = Some(nwd.0[i]);
        }

        // Add the positions to the list.
        self.0.extend(rhs.0);
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

    #[test]
    pub fn test_adjust_position() {
        let eth_price = 1.0;
        let btc_price = 1.0;

        let mut positions = Positions::new(vec![
            Position::new(
                Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
                Some(eth_price),
                Some(0.5),
                Some(Weight::new(0.5).unwrap()),
                None,
            ),
            Position::new(
                Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
                Some(btc_price),
                Some(0.5),
                Some(Weight::new(0.5).unwrap()),
                None,
            ),
        ]);

        let aum = positions.aum();
        let eth_market_value = positions.0[0].market_value();
        let btc_market_value = positions.0[1].market_value();

        let eth_weight_delta = 0.25;
        let btc_weight_delta = -eth_weight_delta;

        positions
            .adjust(
                positions.0[0].weight.unwrap_or_default().id,
                eth_weight_delta,
            )
            .unwrap();

        assert_eq!(
            positions,
            Positions(vec![
                Position::new(
                    Coin::new("Ethereum".to_string(), "ETH".to_string(), 8),
                    Some(eth_price),
                    Some(0.5 + eth_weight_delta),
                    Some(Weight::new(0.5 + eth_weight_delta).unwrap()),
                    None,
                ),
                Position::new(
                    Coin::new("Bitcoin".to_string(), "BTC".to_string(), 8),
                    Some(btc_price),
                    Some(0.5 + btc_weight_delta),
                    Some(Weight::new(0.5 + btc_weight_delta).unwrap()),
                    None,
                ),
            ])
        );

        assert_eq!(positions.aum(), aum);
        assert_eq!(
            positions.0[0].market_value(),
            eth_market_value + eth_weight_delta * eth_price
        );
        assert_eq!(
            positions.0[1].market_value(),
            btc_market_value + btc_weight_delta * btc_price
        );
    }
}
