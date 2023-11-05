//! Traits and types for implementing editable config fields.

/// We want to be able to use the same ConfigFieldType for all config fields,
/// including enums.
pub trait ConfigEnum: std::fmt::Debug {}

/// Each field of a config is converted to one of these types.
/// This allows us to use the same type for all config fields, including enums.
/// Config fields which have their own fields are represented by a Box<dyn
/// Config>.
pub enum ConfigFieldType {
    StringType(String),
    ValueType(f64),
    EnumType(Box<dyn ConfigEnum>),
    ConfigType(Box<dyn Config>),
}

/// Field types can have their unformatted values returned with `raw()`.
impl ConfigFieldType {
    /// Returns the unformatted value of the field.
    pub fn raw(&self) -> String {
        match self {
            ConfigFieldType::StringType(s) => s.clone(),
            ConfigFieldType::ValueType(s) => s.to_string(),
            ConfigFieldType::EnumType(e) => format!("{:?}", e),
            _ => String::new(),
        }
    }
}

/// Each field has its own label and config type, which determines how it is
/// rendered.
pub struct ConfigField {
    pub label: String,
    pub value: ConfigFieldType,
}

/// Nested config fields are a group of individual fields, which has its own
/// name and a field that implements the Config trait.
pub struct NestedConfigField {
    pub name: String,
    pub field: Box<dyn Config>,
}

/// Configs must return a list of ConfigFields, which will render their label
/// and data. When that data is altered, the set_field method is called with the
/// field name and new value. If the config has nested fields, they must be
/// returned by the nested_fields method. When a nested field is altered, the
/// set_nested_field method is called with the nested field name, field name,
/// and new value.
pub trait Config {
    fn fields(&self) -> Vec<ConfigField>;
    fn set_field(&mut self, field_name: String, value: String);
    fn set_nested_field(&mut self, _nested_name: String, _field_name: String, _value: String) {}
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
