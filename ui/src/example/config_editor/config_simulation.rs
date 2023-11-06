//! Implement UI config traits for the simulation config.

use anyhow::{Error, Result};
use simulation::{
    settings::{self, parameters::*},
    simulations::*,
};

use super::config::*;

/// Implements the `ConfigEnum` trait for the `SimulationType` enum so it can be
/// used as a config field.
impl ConfigEnum for SimulationType {}

/// SimulationConfig enum must be converted to a `ConfigFieldType` enum.
impl From<SimulationType> for ConfigFieldType {
    fn from(e: SimulationType) -> Self {
        ConfigFieldType::EnumType(Box::new(e))
    }
}

impl Validatable for SimulationType {
    fn validate(&self, input: &String) -> Result<Self, Error> {
        // Match against the enum variants
        let converted = match input.as_str() {
            "DynamicWeights" => SimulationType::DynamicWeights,
            "StablePortfolio" => SimulationType::StablePortfolio,
            "MomentumStrategy" => SimulationType::MomentumStrategy,
            _ => {
                return Err(Error::msg(format!(
                    "Invalid value for SimulationType: {}",
                    input
                )))
            }
        };
        Ok(converted)
    }
}

/// Implements the Config trait for the SimulationConfig.
impl<P: Parameterized<f64> + Default + std::fmt::Debug> Config for settings::SimulationConfig<P> {
    /// Returns a HashMap of ConfigFields for the SimulationConfig.
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(
            self,
            simulation,
            max_parallel,
            output_directory,
            output_file_name
        )
    }

    fn set_field(&mut self, field_name: String, value: String) -> Result<(), Error> {
        match field_name.as_str() {
            "simulation" => {
                self.simulation = self.simulation.validate(&value)?.into();
            }
            "max_parallel" => {
                if let Some(max_parallel) = &self.max_parallel {
                    self.max_parallel = Some(max_parallel.validate(&value)?.into());
                } else {
                    self.max_parallel = Some((0 as usize).validate(&value)?.into());
                }
            }
            "output_directory" => {
                self.output_directory = self.output_directory.validate(&value)?.into();
            }
            "output_file_name" => {
                if let Some(output_file_name) = &self.output_file_name {
                    self.output_file_name = Some(output_file_name.validate(&value)?.into());
                } else {
                    self.output_file_name = Some(String::new().validate(&value)?.into());
                }
            }
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }
        Ok(())
    }

    fn nested_fields(&self) -> Vec<NestedConfigField> {
        nested_configs!(self.pool)
    }

    fn set_nested_field(
        &mut self,
        nested_name: String,
        field_name: String,
        value: String,
    ) -> Result<(), Error> {
        if nested_name == "pool" {
            match field_name.as_str() {
                "fee_basis_points" => {
                    self.pool.fee_basis_points = self.pool.fee_basis_points.validate(&value)?.into()
                }
                "weight_x" => self.pool.weight_x = self.pool.weight_x.validate(&value)?.into(),
                "target_volatility" => {
                    self.pool.target_volatility =
                        self.pool.target_volatility.validate(&value)?.into()
                }
                _ => tracing::info!(
                    "Attempting nested field name: {} with value: {}",
                    field_name,
                    value
                ),
            }
        } else {
            tracing::info!(
                "Attempting nested field name: {} with value: {}",
                nested_name,
                value
            )
        }

        Ok(())
    }
}

/// Implements Config for the PoolParameters struct.
/// This enables PoolParameters to be a field on a richer config, like
/// SimulationConfig.
impl Config for settings::parameters::PoolParameters {
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(self, fee_basis_points, weight_x, target_volatility)
    }

    fn set_field(&mut self, field_name: String, value: String) -> Result<(), Error> {
        match field_name.as_str() {
            "fee_basis_points" => {
                self.fee_basis_points = self.fee_basis_points.validate(&value)?.into()
            }
            "weight_x" => self.weight_x = self.weight_x.validate(&value)?.into(),
            "target_volatility" => {
                self.target_volatility = self.target_volatility.validate(&value)?.into()
            }
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }

        Ok(())
    }
}
