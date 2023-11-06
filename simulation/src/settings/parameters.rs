use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::Hasher,
};

use ethers::abi::Param;
use serde_json::Value;

use super::*;
use crate::agents::{
    swapper::SwapperParameters,
    weight_changer::{
        momentum::MomentumParameters, volatility_targeting::VolatilityTargetingParameters,
    },
};

pub trait Parameterized {
    fn parameters(&self) -> Vec<f64>;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Single(pub f64);
impl Parameterized for Single {
    fn parameters(&self) -> Vec<f64> {
        vec![self.0]
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Multiple(LinspaceParameters);
impl Parameterized for Multiple {
    fn parameters(&self) -> Vec<f64> {
        self.0.generate()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LinspaceParameters {
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub steps: Option<usize>,
    pub fixed: Option<f64>,
}

impl LinspaceParameters {
    pub fn generate(&self) -> Vec<f64> {
        // Check if start, end, steps are all Some
        match (self.start, self.end, self.steps) {
            (Some(start), Some(end), Some(steps)) => {
                if self.fixed.is_some() {
                    panic!("Both linspace and fixed parameters are set");
                }
                let step_size = (end - start) / (steps as f64 - 1.0);
                (0..steps).map(|i| start + step_size * i as f64).collect()
            }
            // If only fixed is Some, return a vec with that fixed value
            (_, _, _) if self.fixed.is_some() => vec![self.fixed.unwrap()],
            // Otherwise, configuration is invalid
            _ => panic!("Invalid configuration for LinspaceParameters. Please provide a `start`, `end`, and `steps` or alternatively just provide a `fixed` value."),
        }
    }
}

#[macro_export]
macro_rules! linspace {
    ($start:expr, $end:expr, $steps:expr) => {
        LinspaceParameters {
            start: Some($start),
            end: Some($end),
            steps: Some($steps),
            fixed: None,
        }
        .generate()
    };
}
