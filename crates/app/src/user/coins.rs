use std::fs::File;

use datatypes::portfolio::coin_list::CoinList;

use super::*;

const COIN_LIST_EXTENSION: &'static str = "json";
const COIN_LIST_SUFFIX: &'static str = "coinlist";

impl Saveable for CoinList {
    fn prefix(&self) -> Option<String> {
        println!("prefix: {:?} {}", self.name, self.name.clone().is_empty());
        match !self.name.clone().is_empty() {
            true => Some(self.name.clone()),
            false => None,
        }
    }

    fn create_new(name: Option<String>) -> Result<Self> {
        let path = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };

        let file = File::create(path)?;

        let mut coinlist = Self::default();
        match name {
            Some(name) => coinlist.name = name,
            None => {}
        }

        serde_json::to_writer_pretty(file, &coinlist)?;
        Ok(coinlist)
    }

    const EXTENSION: &'static str = COIN_LIST_EXTENSION;
    const SUFFIX: &'static str = COIN_LIST_SUFFIX;
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_coinlist_create() {
        let result = CoinList::create_new(Some("test".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().file_path()).exists());
    }

    #[test]
    fn test_coinlist_load() {
        let name = Some("test".to_string());
        let file_path = CoinList::file_path_with_name(name.unwrap());
        let result = CoinList::load(Some(file_path));
        println!("Loaded coinlist: {:?}", result);
        assert!(result.is_ok());
        assert_eq!("test".to_string(), result.unwrap().name);
    }
}
