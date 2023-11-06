//! Traits and types for implementing editable config fields.

use anyhow::{Error, Result};
use std::fmt::Debug;

/// Every field of a config is represented by a ConfigField.
/// - label: A human-readable name for the field.
/// - data: The field's data, as a ConfigFieldType.
#[derive(Debug)]
pub struct ConfigField {
    pub label: String,
    pub data: ConfigFieldType,
}

impl ConfigField {
    /// Sets the label and data for the field and
    /// converts the data type to its respective ConfigFieldType.
    pub fn new<T: Into<ConfigFieldType>>(label: String, data: T) -> Self {
        ConfigField {
            label,
            data: data.into(),
        }
    }

    /// Returns the inner ConfigFieldType as a String.
    pub fn get_value(&self) -> String {
        self.data.value()
    }
}

/// Each field of a config is converted to one of these types.
/// This allows us to use the same type for all config fields, including enums.
/// Config fields which have their own fields are represented by a Box<dyn
/// Config>.
#[derive(Debug)]
pub enum ConfigFieldType {
    StringType(String),
    ValueType(f64),
    EnumType(Box<dyn ConfigEnum>),
    ConfigType(Box<dyn Config>),
}

impl ConfigFieldType {
    pub fn value(&self) -> String {
        match self {
            ConfigFieldType::StringType(s) => s.clone(),
            ConfigFieldType::ValueType(s) => {
                let converted = format!("{:.51}", s)
                    .trim_end_matches(|c| c == '0' || c == '.')
                    .to_string();

                tracing::info!("Changed original value {} to converted {}", s, converted);
                converted
            }
            ConfigFieldType::EnumType(e) => format!("{:?}", e),
            _ => String::new(),
        }
    }
}

/// We want to be able to use the same ConfigFieldType for all config fields,
/// including enums.
pub trait ConfigEnum: Debug {}

pub trait Validatable: Debug {
    fn validate(&self, input: &String) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Validatable for String {
    fn validate(&self, input: &String) -> Result<Self, Error> {
        // Perform validation for String
        Ok(input.clone())
    }
}

impl Validatable for f64 {
    fn validate(&self, input: &String) -> Result<Self, Error> {
        // If input is empty, return an empty string
        if input.trim().is_empty() {
            return Ok(0.0);
        }

        // Try to parse the input as f64
        let converted = input
            .parse::<f64>()
            .map_err(|_| Error::msg("Invalid value"))?;
        Ok(converted)
    }
}

impl Validatable for usize {
    fn validate(&self, input: &String) -> Result<Self, Error> {
        // Try to parse the input as usize
        let converted = input
            .parse::<usize>()
            .map_err(|_| Error::msg("Invalid value"))?;
        Ok(converted)
    }
}

impl Validatable for u16 {
    fn validate(&self, input: &String) -> Result<Self, Error> {
        // Try to parse the input as u16
        let converted = input
            .parse::<u16>()
            .map_err(|_| Error::msg("Invalid value"))?;
        Ok(converted)
    }
}

/// Nested config fields are a group of individual fields, which has its own
/// name and a field that implements the Config trait.
pub struct NestedConfigField {
    pub name: String,
    pub field: Box<dyn Config>,
}

pub trait Config: Debug {
    fn fields(&self) -> Vec<ConfigField>;

    fn set_field(&mut self, field_name: String, value: String) -> Result<(), Error>;

    fn set_nested_field(
        &mut self,
        _nested_name: String,
        _field_name: String,
        _value: String,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn nested_fields(&self) -> Vec<NestedConfigField> {
        vec![]
    }
}

impl From<u16> for ConfigFieldType {
    fn from(s: u16) -> Self {
        ConfigFieldType::ValueType(s as f64)
    }
}

impl From<u64> for ConfigFieldType {
    fn from(s: u64) -> Self {
        ConfigFieldType::ValueType(s as f64)
    }
}

impl From<String> for ConfigFieldType {
    fn from(s: String) -> Self {
        ConfigFieldType::StringType(s)
    }
}

impl From<f64> for ConfigFieldType {
    fn from(s: f64) -> Self {
        ConfigFieldType::ValueType(s)
    }
}

impl From<Box<dyn Config>> for ConfigFieldType {
    fn from(s: Box<dyn Config>) -> Self {
        ConfigFieldType::ConfigType(s)
    }
}

impl From<Option<usize>> for ConfigFieldType {
    fn from(e: Option<usize>) -> Self {
        ConfigFieldType::ValueType(e.unwrap_or_default() as f64)
    }
}

impl From<Option<String>> for ConfigFieldType {
    fn from(e: Option<String>) -> Self {
        ConfigFieldType::StringType(e.unwrap_or_default())
    }
}

impl From<Option<u16>> for ConfigFieldType {
    fn from(e: Option<u16>) -> Self {
        ConfigFieldType::ValueType(e.unwrap_or_default() as f64)
    }
}
