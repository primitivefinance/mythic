//! Implement UI config traits for the simulation config.

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

/// Implements the Config trait for the SimulationConfig.
impl<P: Parameterized<f64> + Default> Config for settings::SimulationConfig<P> {
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(
            self,
            simulation,
            max_parallel,
            output_directory,
            output_file_name
        )
    }

    fn set_field(&mut self, field_name: String, value: String) {
        match field_name.as_str() {
            "max_parallel" => self.max_parallel = value.parse().ok(),
            "output_directory" => self.output_directory = value,
            "output_file_name" => self.output_file_name = value.parse().ok(),
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }
    }

    fn nested_fields(&self) -> Vec<NestedConfigField> {
        nested_configs!(self.pool)
    }

    fn set_nested_field(&mut self, nested_name: String, field_name: String, value: String) {
        if nested_name == "pool" {
            match field_name.as_str() {
                "fee_basis_points" => {
                    self.pool.fee_basis_points = value.parse().ok().unwrap_or_default()
                }
                "weight_x" => self.pool.weight_x = value.parse().ok().unwrap_or_default(),
                "target_volatility" => {
                    self.pool.target_volatility = value.parse().ok().unwrap_or_default()
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
    }
}

/// Implements Config for the PoolParameters struct.
/// This enables PoolParameters to be a field on a richer config, like
/// SimulationConfig.
impl Config for settings::parameters::PoolParameters {
    fn fields(&self) -> Vec<ConfigField> {
        config_fields!(self, fee_basis_points, weight_x, target_volatility)
    }

    fn set_field(&mut self, field_name: String, value: String) {
        match field_name.as_str() {
            "fee_basis_points" => self.fee_basis_points = value.parse().ok().unwrap_or_default(),
            "weight_x" => self.weight_x = value.parse().ok().unwrap_or_default(),
            "target_volatility" => self.target_volatility = value.parse().ok().unwrap_or_default(),
            _ => tracing::info!(
                "Attempting field name: {} with value: {}",
                field_name,
                value
            ),
        }
    }
}
