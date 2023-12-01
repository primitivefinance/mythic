//! Main serialized data structure for Portfolios.

use datatypes::portfolio::Portfolio;

use super::*;

const PORTFOLIO_EXTENSION: &'static str = "json";
const PORTFOLIO_SUFFIX: &'static str = "portfolio";

impl Saveable for Portfolio {
    const EXTENSION: &'static str = PORTFOLIO_EXTENSION;
    const SUFFIX: &'static str = PORTFOLIO_SUFFIX;

    fn prefix(&self) -> Option<String> {
        Some(self.name.clone())
    }

    /// Overwrites the default dir (just config_dir) with a new directory
    /// "portfolios".
    fn dir() -> PathBuf {
        // If portfolios directory does not exist, create it.
        let dir = Self::config_dir().join("portfolios");
        if !dir.exists() {
            std::fs::create_dir_all(&dir).expect("Could not create portfolios directory.");
        }

        dir
    }

    fn create_new(name: Option<String>) -> Result<Self> {
        let name = name.unwrap_or_else(|| "default".to_string());
        let path = Self::dir().join(format!("{}.{}.{}", name, Self::SUFFIX, Self::EXTENSION));
        let file = File::create(path)?;

        let mut portfolio = Self::default();
        portfolio.name = name;

        serde_json::to_writer_pretty(file, &portfolio)?;
        Ok(portfolio)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_portfolio_create() {
        let result = Portfolio::create_new(Some("test".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().file_path()).exists());
    }

    #[test]
    fn test_portfolio_load() {
        let name = Some("test".to_string());
        let file_path = Portfolio::file_path_with_name(name.unwrap());
        let result = Portfolio::load(Some(file_path));
        println!("Loaded portfolio: {:?}", result);
        assert!(result.is_ok());
        assert_eq!("test".to_string(), result.unwrap().name);
    }
}
