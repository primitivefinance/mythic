use std::{borrow::Cow, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// A category classification defined by the user for user experience.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Hash)]
pub enum Category {
    Trusted,
    #[default]
    Untrusted,
    Blocked,
    Recent,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::Trusted,
            Category::Untrusted,
            Category::Blocked,
            Category::Recent,
        ]
    }
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Trusted" => Ok(Category::Trusted),
            "Untrusted" => Ok(Category::Untrusted),
            "Blocked" => Ok(Category::Blocked),
            "Recent" => Ok(Category::Recent),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Category::Trusted => "Trusted",
                Category::Untrusted => "Untrusted",
                Category::Blocked => "Blocked",
                Category::Recent => "Recent",
            }
        )
    }
}

impl From<Cow<'_, str>> for Category {
    fn from(item: Cow<'_, str>) -> Self {
        match item.as_ref() {
            "Trusted" => Category::Trusted,
            "Untrusted" => Category::Untrusted,
            "Blocked" => Category::Blocked,
            "Recent" => Category::Recent,
            _ => Category::Untrusted,
        }
    }
}

/// A user defined type for an address, not enforced or validated.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Class {
    #[default]
    EOA,
    Contract,
    AccountAbstraction,
}

impl Class {
    pub fn all() -> Vec<Class> {
        vec![Class::EOA, Class::Contract, Class::AccountAbstraction]
    }
}

impl FromStr for Class {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EOA" => Ok(Class::EOA),
            "Contract" => Ok(Class::Contract),
            "AccountAbstraction" => Ok(Class::AccountAbstraction),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Class::EOA => "EOA",
                Class::Contract => "Contract",
                Class::AccountAbstraction => "AccountAbstraction",
            }
        )
    }
}

impl From<Cow<'_, str>> for Class {
    fn from(item: Cow<'_, str>) -> Self {
        match item.as_ref() {
            "EOA" => Class::EOA,
            "Contract" => Class::Contract,
            "AccountAbstraction" => Class::AccountAbstraction,
            _ => Class::EOA,
        }
    }
}
