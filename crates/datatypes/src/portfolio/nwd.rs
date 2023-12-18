//! NWD or Normalized Weight Distribution is a composition of weights that sum
//! to 1.0. This data structure is used to represent the distribution of
//! weights for a portfolio.

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::weight::*;

/// Normalized Weight Distribution
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NWD(pub Vec<Weight>);

impl NWD {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn sum(&self) -> Weight {
        self.0.iter().fold(Weight::default(), |acc, x| acc + *x)
    }

    pub fn validate(&self) -> Result<(), WeightError> {
        let sum = self.sum();
        if sum != MAX_WEIGHT {
            return Err(WeightError::InvalidSum(sum.into()));
        }
        Ok(())
    }

    /// Formula for:
    ///
    /// Changing each unadjusted weight:
    /// w' = w - delta / (self.len() - 1)
    ///
    /// Changing the adjusted weight:
    /// w' = w + delta
    ///
    /// Arguments:
    /// id - the id of the weight to adjust
    /// delta - the amount to adjust the weight by, which can be negative.
    #[allow(unused_parens)]
    pub fn adjust(&mut self, id: Uuid, delta: f64) -> Result<(), WeightError> {
        // Validate the delta is within valid bounds.
        let absolute = delta.abs();
        if !(MIN_WEIGHT.value..=MAX_WEIGHT.value).contains(&absolute) {
            return Err(WeightError::InvalidWeight(absolute));
        }

        // Find the weight that is being adjusted, if it exists.
        let inside = self.0.iter().find(|w| w.id == id).cloned();

        // We proportionally adjust all elements.
        let mut divisor = self.len();

        // If the weight that is being adjusted is already in the list,
        // then we reduce the divisor by 1 to exclude it from the
        // other weights that are being adjusted.
        if inside.is_some() {
            divisor -= 1;
        }

        // Adjust all weights.
        for w in self.0.iter_mut() {
            if w.id == id {
                *w += delta;
            } else {
                *w -= delta / (divisor as f64);
            }
        }

        // If the delta is new, add it to the list
        if inside.is_none() && delta != 0.0 {
            self.0.push(Weight { id, value: delta });
        }

        // Validate the sum of the weights is equal to 1.0 within the f64 epsilon.
        self.validate()?;

        Ok(())
    }
}

/// Add a weight to the distribution.
/// Adding a weight will require all weights to be proportionally reduced.
impl Add<Weight> for NWD {
    type Output = Self;

    fn add(self, rhs: Weight) -> Self::Output {
        let mut nwd = self.clone();
        nwd.adjust(rhs.id, rhs.value).unwrap();
        nwd
    }
}

impl Add<NWD> for NWD {
    type Output = Self;

    fn add(self, rhs: NWD) -> Self::Output {
        let mut nwd = self.clone();
        for w in rhs.0 {
            nwd.adjust(w.id, w.value).unwrap();
        }
        nwd
    }
}

/// Subtract a weight from the distribution.
/// Subtracting a weight will require all weights to be proportionally
/// increased.
impl Sub<Weight> for NWD {
    type Output = Self;

    fn sub(self, rhs: Weight) -> Self::Output {
        let mut nwd = self.clone();
        nwd.adjust(rhs.id, rhs.value.neg()).unwrap();
        nwd
    }
}

impl AddAssign<Weight> for NWD {
    fn add_assign(&mut self, rhs: Weight) {
        self.adjust(rhs.id, rhs.value).unwrap();
    }
}

impl AddAssign<NWD> for NWD {
    fn add_assign(&mut self, rhs: NWD) {
        for w in rhs.0 {
            self.adjust(w.id, w.value).unwrap();
        }
    }
}

/// todo: decide the behavior of this. If we reduce a weight all the way to 0.0,
/// should we remove it from the list?
impl SubAssign<Weight> for NWD {
    fn sub_assign(&mut self, rhs: Weight) {
        self.adjust(rhs.id, rhs.value.neg()).unwrap();
    }
}

impl SubAssign<NWD> for NWD {
    fn sub_assign(&mut self, rhs: NWD) {
        for w in rhs.0 {
            self.adjust(w.id, w.value.neg()).unwrap();
        }
    }
}

impl AddAssign<f64> for NWD {
    fn add_assign(&mut self, rhs: f64) {
        self.adjust(Uuid::new_v4(), rhs).unwrap();
    }
}

impl SubAssign<f64> for NWD {
    fn sub_assign(&mut self, rhs: f64) {
        self.adjust(Uuid::new_v4(), rhs.neg()).unwrap();
    }
}

impl From<Vec<Weight>> for NWD {
    fn from(weights: Vec<Weight>) -> Self {
        let mut nwd = NWD(vec![]);
        for w in weights {
            nwd += w;
        }
        nwd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nwd_add() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd.adjust(weight.id, 0.25).unwrap();
        assert_eq!(
            nwd,
            NWD(vec![Weight::new(0.75).unwrap(), Weight::new(0.25).unwrap()])
        );
    }

    #[test]
    fn test_nwd_sub() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd.adjust(weight.id, 0.25.neg()).unwrap();
        assert_eq!(
            nwd,
            NWD(vec![Weight::new(0.25).unwrap(), Weight::new(0.75).unwrap()])
        );
    }

    #[test]
    fn test_nwd_add_weight() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd += weight;
        assert_eq!(
            nwd,
            NWD(vec![Weight::new(1.0).unwrap(), Weight::new(0.0).unwrap()])
        );
    }

    #[test]
    fn test_nwd_sub_weight() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd -= weight;
        assert_eq!(
            nwd,
            NWD(vec![Weight::new(0.0).unwrap(), Weight::new(1.0).unwrap()])
        );
    }

    #[test]
    fn test_nwd_add_additional_weight() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd += 0.2;
        assert_eq!(
            nwd,
            NWD(vec![
                Weight::new(0.4).unwrap(),
                Weight::new(0.4).unwrap(),
                Weight::new(0.2).unwrap()
            ])
        );
    }

    #[test]
    fn test_nwd_sub_out_weight() {
        let weight = Weight::new(0.5).unwrap();
        let mut nwd = NWD(vec![weight, Weight::new(0.5).unwrap()]);
        nwd -= weight;
        assert_eq!(
            nwd,
            NWD(vec![Weight::new(0.0).unwrap(), Weight::new(1.0).unwrap()])
        );
    }
}
