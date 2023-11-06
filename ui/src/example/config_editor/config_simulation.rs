//! Implement UI config traits for the simulation config.

use anyhow::{Error, Result};
use simulation::{
    settings::{self, parameters::*},
    simulations::*,
};

use super::{config::*, config_ui::*};

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

impl<P> From<Store> for Result<settings::SimulationConfig<P>, Error>
where
    P: Parameterized<f64> + Default + std::fmt::Debug + Clone + 'static,
{
    fn from(s: Store) -> Self {
        let mut settings = settings::SimulationConfig::<P>::default();
        let simulation = s
            .get("simulation")
            .ok_or_else(|| Error::msg("Expected simulation field"))?;

        let simulation = match &simulation {
            StoreField::Value(s) => settings.simulation.validate(s)?,
            _ => return Err(Error::msg("Expected simulation field")),
        };

        let output_directory = s
            .get("output_directory")
            .ok_or_else(|| Error::msg("Expected output_directory field"))?;

        let output_directory = match &output_directory {
            StoreField::Value(s) => settings.output_directory.validate(s)?,
            _ => return Err(Error::msg("Expected output_directory field")),
        };

        let output_file_name = s
            .get("output_file_name")
            .map(|output_file_name| match output_file_name {
                StoreField::Value(s) => Some(
                    settings
                        .output_file_name
                        .unwrap_or_default()
                        .validate(s)
                        .unwrap_or_default(),
                ),
                _ => None,
            })
            .unwrap_or_default();

        let pool = s.get("pool").map(|pool| match pool {
            StoreField::Nested(nested) => {
                let mut pool = settings.pool.clone();

                for (field_name, value) in nested.iter() {
                    match field_name.as_str() {
                        "fee_basis_points" => match value {
                            StoreField::Value(s) => {
                                pool.fee_basis_points = pool.fee_basis_points.validate(s)?
                            }
                            _ => return Err(Error::msg("Expected fee_basis_points field")),
                        },
                        "weight_x" => match value {
                            StoreField::Value(s) => pool.weight_x = pool.weight_x.validate(s)?,
                            _ => return Err(Error::msg("Expected weight_x field")),
                        },
                        "target_volatility" => match value {
                            StoreField::Value(s) => {
                                pool.target_volatility = pool.target_volatility.validate(s)?
                            }
                            _ => return Err(Error::msg("Expected target_volatility field")),
                        },
                        _ => tracing::info!("Attempting field name: {}", field_name),
                    }
                }

                Ok(pool)
            }
            _ => Err(Error::msg("Expected pool field")),
        });

        settings.simulation = simulation;
        settings.output_directory = output_directory;
        settings.output_file_name = output_file_name;

        if let Some(pool) = pool {
            settings.pool = pool?;
        }

        Ok(settings)
    }
}
impl<P> From<settings::SimulationConfig<P>> for Store
where
    P: Parameterized<f64> + Default + std::fmt::Debug + Clone + 'static,
{
    fn from(s: settings::SimulationConfig<P>) -> Self {
        let config: Box<dyn Config> = Box::new(s) as Box<dyn Config>;
        config.into()
    }
}

/// Implements the Config trait for the SimulationConfig.
impl<P> Config for settings::SimulationConfig<P>
where
    P: Parameterized<f64> + Default + std::fmt::Debug + Clone + 'static,
{
    fn clone_config(&self) -> Box<dyn Config> {
        Box::new(self.clone())
    }

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
    fn clone_config(&self) -> Box<dyn Config> {
        Box::new(self.clone())
    }

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
