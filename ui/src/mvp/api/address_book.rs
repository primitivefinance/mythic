//! Manages a list of known addresses which is saved in the global application
//! state.

use std::collections::HashMap;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AddressBook {
    pub addresses: HashMap<String, Address>,
}

impl AddressBook {
    pub fn new() -> Self {
        Self {
            addresses: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, address: Address) {
        self.addresses.insert(name, address);
    }

    pub fn get(&self, name: &str) -> Option<Address> {
        self.addresses.get(name).cloned()
    }

    pub fn remove(&mut self, name: &str) {
        self.addresses.remove(name);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AddressBookCategory {
    Trusted,
    #[default]
    Untrusted,
    Blocked,
    Recent,
}

/// Manages a list of known addresses across a few categorized [`AddressBook`]s.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AddressBookManager {
    pub trusted: AddressBook,
    pub untrusted: AddressBook,
    pub blocked: AddressBook,
    pub recent: AddressBook,
}

impl AddressBookManager {
    pub fn new() -> Self {
        Self {
            trusted: AddressBook::new(),
            untrusted: AddressBook::new(),
            blocked: AddressBook::new(),
            recent: AddressBook::new(),
        }
    }

    pub fn add(&mut self, name: String, address: Address, category: AddressBookCategory) {
        match category {
            AddressBookCategory::Trusted => self.trusted.add(name, address),
            AddressBookCategory::Untrusted => self.untrusted.add(name, address),
            AddressBookCategory::Blocked => self.blocked.add(name, address),
            AddressBookCategory::Recent => self.recent.add(name, address),
        }
    }

    pub fn get(&self, name: &str, category: AddressBookCategory) -> Option<Address> {
        match category {
            AddressBookCategory::Trusted => self.trusted.get(name),
            AddressBookCategory::Untrusted => self.untrusted.get(name),
            AddressBookCategory::Blocked => self.blocked.get(name),
            AddressBookCategory::Recent => self.recent.get(name),
        }
    }

    pub fn remove(&mut self, name: &str, category: AddressBookCategory) {
        match category {
            AddressBookCategory::Trusted => self.trusted.remove(name),
            AddressBookCategory::Untrusted => self.untrusted.remove(name),
            AddressBookCategory::Blocked => self.blocked.remove(name),
            AddressBookCategory::Recent => self.recent.remove(name),
        }
    }

    pub fn list(&self, category: AddressBookCategory) -> Vec<(String, Address)> {
        match category {
            AddressBookCategory::Trusted => self.trusted.addresses.clone().into_iter().collect(),
            AddressBookCategory::Untrusted => {
                self.untrusted.addresses.clone().into_iter().collect()
            }
            AddressBookCategory::Blocked => self.blocked.addresses.clone().into_iter().collect(),
            AddressBookCategory::Recent => self.recent.addresses.clone().into_iter().collect(),
        }
    }

    pub fn clear(&mut self, category: AddressBookCategory) {
        match category {
            AddressBookCategory::Trusted => self.trusted.addresses.clear(),
            AddressBookCategory::Untrusted => self.untrusted.addresses.clear(),
            AddressBookCategory::Blocked => self.blocked.addresses.clear(),
            AddressBookCategory::Recent => self.recent.addresses.clear(),
        }
    }
}
