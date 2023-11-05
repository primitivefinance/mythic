//! Macros for generating config fields.

macro_rules! nested_configs {
    ($($field:expr),* $(,)?) => {
        vec![
            $(
                NestedConfigField {
                    name: format!("{}", stringify!($field).split('.').last().unwrap_or("").to_string()),
                    field: Box::new($field.clone()),
                },
            )*
        ]
    };
}

macro_rules! config_fields {
    ($self:expr, $($field:ident),* $(,)?) => {
        vec![
            $(
                ConfigField {
                    label: stringify!($field).to_string(),
                    value: ConfigFieldType::from($self.$field.clone()),
                },
            )*
        ]
    };
}
