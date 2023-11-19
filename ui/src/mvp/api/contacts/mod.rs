//! Mini service for organizing and managing contacts.
//! Contacts are any address that the user wants to keep track of.
//! Contacts are organized into categories, which are defined by the
//! `Category` enum.

pub mod classification;
pub mod list;

use std::{
    collections::HashMap,
    io::{Read, Write},
};

pub use classification::*;
use ethers::prelude::*;
pub use list::*;

#[derive(Clone, Debug, Default)]
pub struct Contacts {
    pub books: HashMap<Category, ContactList>,
}

impl Contacts {
    pub fn new() -> Self {
        let mut books = HashMap::new();
        books.insert(Category::Trusted, ContactList::new());
        books.insert(Category::Untrusted, ContactList::new());
        books.insert(Category::Blocked, ContactList::new());
        books.insert(Category::Recent, ContactList::new());

        Self { books }
    }

    pub fn add(&mut self, address: Address, label: ContactValue, category: Category) {
        if let Some(book) = self.books.get_mut(&category) {
            book.add(address, label);
        }
    }

    pub fn get(&self, address: &Address, category: Category) -> Option<&ContactValue> {
        self.books.get(&category)?.get(address)
    }

    pub fn get_list(&self, category: Category) -> Option<&ContactList> {
        self.books.get(&category)
    }

    pub fn remove(&mut self, address: &Address, category: Category) {
        if let Some(book) = self.books.get_mut(&category) {
            book.remove(address);
        }
    }

    pub fn list(&self, category: Category) -> Vec<(&Address, &ContactValue)> {
        self.books
            .get(&category)
            .map(|book| book.get_all())
            .unwrap_or_else(Vec::new)
    }

    pub fn clear(&mut self, category: Category) {
        if let Some(book) = self.books.get_mut(&category) {
            book.clear();
        }
    }

    pub fn try_add(
        &mut self,
        address: String,
        contact: ContactValue,
        category: Category,
    ) -> anyhow::Result<(), anyhow::Error> {
        let address = address
            .parse::<Address>()
            .map_err(|e| anyhow::anyhow!("Failed to parse address: {}", e.to_string()))?;

        self.add(address, contact, category);
        Ok(())
    }

    pub fn load_from_file(&mut self, category: Category, path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let contacts: Vec<(Address, ContactValue)> = serde_json::from_str(&contents)?;
        for (address, contact) in contacts {
            self.add(address, contact, category.clone());
        }
        Ok(())
    }

    pub fn save_to_file(&self, category: Category, path: &str) -> Result<(), std::io::Error> {
        let contacts = self.list(category);
        let mut file = std::fs::File::create(path)?;
        let json = serde_json::to_string(&contacts)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
