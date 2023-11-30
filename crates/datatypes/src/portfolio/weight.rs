//! A portfolio weight always sums to one, so operating on them should be
//! mindful of that.

use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A f64 value with a unique id that when grouped will always be a in a group
/// that sums to 1.0.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Weight {
    pub id: Uuid,
    pub value: f64,
}

pub const MAX_WEIGHT: Weight = Weight {
    id: Uuid::nil(),
    value: 1.0,
};
pub const MIN_WEIGHT: Weight = Weight {
    id: Uuid::nil(),
    value: 0.0,
};
pub const EPSILON_WEIGHT: Weight = Weight {
    id: Uuid::nil(),
    value: f64::EPSILON,
};

impl Weight {
    pub fn new(weight: f64) -> Result<Self, WeightError> {
        let clamped_weight = weight.clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        // If these values are not the same it means the weight is out of bounds.
        if (clamped_weight - weight).abs() > f64::EPSILON {
            Err(WeightError::InvalidWeight(weight))
        } else {
            Ok(Weight {
                id: Uuid::new_v4(),
                value: weight,
            })
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
            ..*self
        }
    }

    pub fn adjust(&mut self, adjustment: f64) {
        self.value = (self.value + adjustment).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl Add for Weight {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Check if we overflow our upper bound or underflow our lower bound.
        let sum = (self.value + rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        println!("Addding {} and {}: {} ", self.value, rhs.value, sum);
        // If rhs has the same id as self, then we can just add the values
        // together.
        if self.id == rhs.id {
            return Self { value: sum, ..self };
        }

        // Else, add them and return a weight with a new id.
        Weight::new(sum).unwrap()
    }
}

impl Sub for Weight {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let difference = (self.value - rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        // If rhs has the same id as self, then we can just subtract the values
        // together.
        if self.id == rhs.id {
            return Self {
                value: difference,
                ..self
            };
        }

        // Else, subtract them and return a weight with a new id.
        Weight::new(difference).unwrap()
    }
}

impl Mul for Weight {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = (self.value * rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        // If rhs has the same id as self, then we can just multiply the values
        // together.
        if self.id == rhs.id {
            return Self {
                value: product,
                ..self
            };
        }

        // Else, return a new weight with a new id.
        Weight::new(product).unwrap()
    }
}

impl Div for Weight {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let quotient = (self.value / rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        // If rhs has the same id as self, then we can just divide the values
        // together.
        if self.id == rhs.id {
            return Self {
                value: quotient,
                ..self
            };
        }

        // Else, return a new weight with a new id.
        Weight::new(quotient).unwrap()
    }
}

impl Add<f64> for Weight {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let sum = (self.value + rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        Self { value: sum, ..self }
    }
}

impl Sub<f64> for Weight {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        let difference = (self.value - rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        Self {
            value: difference,
            ..self
        }
    }
}

impl Div<f64> for Weight {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let quotient = (self.value / rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
        Self {
            value: quotient,
            ..self
        }
    }
}

impl Mul<f64> for Weight {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let product = (self.value * rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);

        Self {
            value: product,
            ..self
        }
    }
}

impl AddAssign<f64> for Weight {
    fn add_assign(&mut self, rhs: f64) {
        self.value = (self.value + rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl AddAssign for Weight {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl SubAssign<f64> for Weight {
    fn sub_assign(&mut self, rhs: f64) {
        self.value = (self.value - rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl SubAssign for Weight {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value - rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl MulAssign<f64> for Weight {
    fn mul_assign(&mut self, rhs: f64) {
        self.value = (self.value * rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl MulAssign for Weight {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl DivAssign<f64> for Weight {
    fn div_assign(&mut self, rhs: f64) {
        self.value = (self.value / rhs).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl DivAssign for Weight {
    fn div_assign(&mut self, rhs: Self) {
        self.value = (self.value / rhs.value).clamp(MIN_WEIGHT.value, MAX_WEIGHT.value);
    }
}

impl From<f64> for Weight {
    fn from(value: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
        }
    }
}

impl From<Weight> for f64 {
    fn from(weight: Weight) -> Self {
        weight.value
    }
}

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Weight {}

impl Hash for Weight {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Weight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Neg for Weight {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            ..self
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Errors that can occur when handling weights.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeightError {
    WeightNotFound(Uuid),
    InvalidWeight(f64),
    InvalidSum(f64),
}

impl fmt::Display for WeightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeightError::WeightNotFound(id) => write!(f, "Weight with id {} not found", id),
            WeightError::InvalidWeight(weight) => {
                write!(f, "Weight {} is not within bounds", weight)
            }
            WeightError::InvalidSum(sum) => write!(f, "Sum {} is not 1.0", sum),
        }
    }
}

impl std::error::Error for WeightError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_new() {
        let weight = Weight::new(0.5).unwrap();
        assert_eq!(weight.value, 0.5);
    }

    #[test]
    fn test_weight_new_invalid() {
        let weight = Weight::new(1.5);
        assert!(weight.is_err());
    }

    #[test]
    fn test_weight_add() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = weight.clone();
        let weight3 = weight + weight2;
        assert_eq!(weight3.value, 1.0);
    }

    #[test]
    fn test_weight_add_different_ids() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = Weight::new(0.5).unwrap();
        let weight3 = weight + weight2;
        assert_ne!(weight3.id, weight.id);
        assert_ne!(weight3.id, weight2.id);
    }

    #[test]
    fn test_weight_add_different_ids_different_values() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = Weight::new(0.5).unwrap();
        let weight3 = weight + weight2;
        assert_ne!(weight3.id, weight.id);
        assert_ne!(weight3.id, weight2.id);
        assert_eq!(weight3.value, 1.0);
    }

    #[test]
    fn test_weight_sub() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = weight.clone();
        let weight3 = weight - weight2;
        assert_eq!(weight3.value, 0.0);
    }

    #[test]
    fn test_weight_sub_different_ids() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = Weight::new(0.5).unwrap();
        let weight3 = weight - weight2;
        assert_ne!(weight3.id, weight.id);
        assert_ne!(weight3.id, weight2.id);
    }

    #[test]
    fn test_weight_sub_different_ids_different_values() {
        let weight = Weight::new(0.5).unwrap();
        let weight2 = Weight::new(0.5).unwrap();
        let weight3 = weight - weight2;
        assert_ne!(weight3.id, weight.id);
        assert_ne!(weight3.id, weight2.id);
        assert_eq!(weight3.value, 0.0);
    }
}
