//! Config store is used as an intermediary container for quickly looking up and
//! editing fields of the Config.

use std::collections::HashMap;

use super::config::*;

/// A config can have a field that is a value, or a nested group of more fields.
#[derive(Debug, Clone)]
pub enum StoreField {
    Value(String),
    Nested(ConfigStore),
}

/// A recursive data type to hold fields and nested fields.
#[derive(Debug, Clone)]
pub struct ConfigStore(pub HashMap<String, StoreField>);

impl From<String> for StoreField {
    fn from(s: String) -> Self {
        StoreField::Value(s)
    }
}

impl From<StoreField> for String {
    fn from(s: StoreField) -> Self {
        match s {
            StoreField::Value(s) => s,
            StoreField::Nested(nested) => {
                let mut formatted_string = String::new();
                for (field_name, value) in nested.0.iter() {
                    formatted_string.push_str(&format!(
                        "{}: {}\n",
                        field_name,
                        String::from(value.clone())
                    ));
                }

                formatted_string
            }
        }
    }
}

impl From<ConfigStore> for StoreField {
    fn from(s: ConfigStore) -> Self {
        StoreField::Nested(s)
    }
}

/// Converts a Config into a ConfigStore.
impl From<Box<dyn Config>> for ConfigStore {
    fn from(c: Box<dyn Config>) -> Self {
        let mut store = ConfigStore(HashMap::new());

        // Populate the store with the current values of the config.
        for field in c.fields().iter() {
            let value = field.get_value().clone();
            store.0.insert(field.label.clone(), value.clone().into());
        }

        // Populate the store with all the nested fields of the config.
        for nested in c.nested_fields().iter() {
            let mut nested_field_store = ConfigStore(HashMap::new());
            for field in nested.field.fields().iter() {
                let value = field.get_value().clone();
                nested_field_store
                    .0
                    .insert(field.label.clone(), value.clone().into());
            }

            store
                .0
                .insert(nested.name.clone(), nested_field_store.into());
        }

        store
    }
}
