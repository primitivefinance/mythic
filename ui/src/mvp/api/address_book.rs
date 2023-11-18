//! Manages a list of known addresses which is saved in the global application
//! state.

use std::collections::HashMap;

use super::*;

type AddressLabel = String;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AddressBook {
    pub addresses: HashMap<Address, AddressLabel>,
}

impl AddressBook {
    pub fn new() -> Self {
        Self {
            addresses: HashMap::new(),
        }
    }

    pub fn add(&mut self, address: Address, label: AddressLabel) {
        self.addresses.insert(address, label);
    }

    pub fn get(&self, address: &Address) -> Option<&AddressLabel> {
        self.addresses.get(address)
    }

    pub fn remove(&mut self, address: &Address) {
        self.addresses.remove(address);
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

    pub fn add(&mut self, address: Address, label: AddressLabel, category: AddressBookCategory) {
        match category {
            AddressBookCategory::Trusted => self.trusted.add(address, label),
            AddressBookCategory::Untrusted => self.untrusted.add(address, label),
            AddressBookCategory::Blocked => self.blocked.add(address, label),
            AddressBookCategory::Recent => self.recent.add(address, label),
        }
    }

    pub fn get(&self, address: &Address, category: AddressBookCategory) -> Option<&AddressLabel> {
        match category {
            AddressBookCategory::Trusted => self.trusted.get(address),
            AddressBookCategory::Untrusted => self.untrusted.get(address),
            AddressBookCategory::Blocked => self.blocked.get(address),
            AddressBookCategory::Recent => self.recent.get(address),
        }
    }

    pub fn remove(&mut self, address: &Address, category: AddressBookCategory) {
        match category {
            AddressBookCategory::Trusted => self.trusted.remove(address),
            AddressBookCategory::Untrusted => self.untrusted.remove(address),
            AddressBookCategory::Blocked => self.blocked.remove(address),
            AddressBookCategory::Recent => self.recent.remove(address),
        }
    }

    pub fn list(&self, category: AddressBookCategory) -> Vec<(Address, AddressLabel)> {
        match category {
            AddressBookCategory::Trusted => self.trusted.addresses.clone().into_iter().collect(),
            AddressBookCategory::Untrusted => {
                self.untrusted.addresses.clone().into_iter().collect()
            }
            AddressBookCategory::Blocked => self.blocked.addresses.clone().into_iter().collect(),
            AddressBookCategory::Recent => self.recent.addresses.clone().into_iter().collect(),
        }
    }

    pub fn list_sorted(&self, category: AddressBookCategory) -> Vec<(Address, AddressLabel)> {
        let mut list = self.list(category);
        list.sort_by(|(a, _), (b, _)| a.cmp(b));
        list
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
