//! Save and load profiles from disk. This contains all the information a user
//! needs to save and load their Excalibur workspace.

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use datatypes::portfolio::{coin_list::CoinList, Portfolio};
use directories_next::{self, ProjectDirs};
use serde::{de::DeserializeOwned, Serialize};
use tracing;

use super::{contacts::Contacts, rpcs::RPCList};
use crate::app::AnvilSave;

pub const PROFILE_FILE_EXTENSION: &str = "json";
pub const PROFILE_FILE_NAME: &str = "profile";
pub const QUALIFIER: &str = "xyz.primitive";
pub const ORGANIZATION: &str = "Primitive";
pub const APPLICATION: &str = "Excalibur";

pub fn get_project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
}

pub fn get_config_dir() -> PathBuf {
    check_in_executable_dir().unwrap_or_else(|| {
        let dir = directories_next::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .expect("Failed to find valid config directory.");
        dir.config_dir().to_path_buf()
    })
}

pub fn get_data_dir() -> PathBuf {
    check_in_executable_dir().unwrap_or_else(|| {
        let dir = directories_next::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .expect("Failed to find valid data directory.");
        dir.data_dir().to_path_buf()
    })
}

/// Checks for a profile executable's directory, if it exists we use that for
/// data + config dirs.
fn check_in_executable_dir() -> Option<PathBuf> {
    let base = directories_next::BaseDirs::new()?;
    let dir = base.executable_dir()?;

    dir.join(PROFILE_FILE_NAME)
        .is_file()
        .then(|| dir.to_path_buf())
}

pub trait Saveable: Serialize + DeserializeOwned + Sized {
    /// File extension of the file.
    const EXTENSION: &'static str = "json";
    /// File suffix of the file. e..g `config` for `config.json`.
    const SUFFIX: &'static str;

    /// Creates a new instance of this type and writes it to disk with `name`.
    /// todo: be careful about overwriting.
    fn create_new(name: Option<String>) -> Result<Self>;

    /// Gets the directory which stores the project's application and config
    /// data.
    fn org_dir() -> PathBuf {
        Self::app_dir()
            .as_path()
            .parent()
            .expect("Could not get parent directory of app.")
            .to_path_buf()
    }

    /// Gets the directory which stores application data..
    fn app_dir() -> PathBuf {
        let dir = get_project_dir().expect("Failed to get project directories.");
        dir.data_dir()
            .parent()
            .expect("Could not get parent of data dir.")
            .to_path_buf()
    }

    /// Gets the directory which stores configuration data of the project.
    fn config_dir() -> PathBuf {
        let dir = get_config_dir();

        // Handle better directory creation for other systems.
        if !dir.exists() {
            println!("Creating profile directory: {:?}", dir.as_path());
            std::fs::create_dir(dir.as_path()).expect("Failed to create profile directory.");
        }

        dir
    }

    fn dir() -> PathBuf {
        Self::config_dir()
    }

    /// Formats the file to have a consistent ending, so it can be validated
    /// when loading.
    fn file_name_ending() -> String {
        format!("{}.{}", Self::SUFFIX, Self::EXTENSION)
    }

    /// Formatted path of these file _types_ in the config directory.
    fn path() -> PathBuf {
        let formatted = Self::file_name_ending();
        Self::dir().join(formatted)
    }

    /// Path of the file of this instance.
    fn file_path(&self) -> PathBuf {
        let mut formatted = Self::file_name_ending();

        if let Some(prefix) = self.prefix() {
            formatted = format!("{}.{}", prefix, formatted);
        }

        let path = Self::dir().join(formatted);

        path
    }

    fn file_path_with_name(name: String) -> PathBuf {
        let mut formatted = Self::file_name_ending();

        formatted = format!("{}.{}", name, formatted);

        let path = Self::dir().join(formatted);

        path
    }

    /// A prefix to add to the file, e.g. `custom` for `custom.profile.json`.
    fn prefix(&self) -> Option<String> {
        None
    }

    /// Writes the file to disk.
    fn save(&self) -> Result<()> {
        let path = self.file_path();
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    /// Loads the file from disk into an instance of this type.
    fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = match path {
            Some(path) => path,
            None => Self::path(),
        };

        // If the file doesn't exist, create a new instance.
        if !path.exists() {
            // return Err(anyhow::anyhow!("Could not find file at path: {:?}", path));
            tracing::trace!("Creating new instance of {}.", Self::SUFFIX);
            let instance = Self::create_new(None)?;
            instance.save()?;
            return Ok(instance);
        }

        tracing::trace!("Loading {} at path: {:?}", Self::SUFFIX, path);
        let file = File::open(path)?;

        let instance = serde_json::from_reader(file)?;

        Ok(instance)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub name: Option<String>,
    pub contacts: Contacts,
    pub rpcs: RPCList,
    pub coins: CoinList,
    pub portfolio: Portfolio,
    pub anvil_snapshot: Option<AnvilSave>,
}

impl UserProfile {
    /// Updates the portfolio of this profile and saves to file.
    pub fn update_portfolio(&mut self, portfolio: &Portfolio) -> Result<()> {
        self.portfolio = portfolio.clone();
        Ok(())
    }
}

impl Saveable for UserProfile {
    const EXTENSION: &'static str = PROFILE_FILE_EXTENSION;
    const SUFFIX: &'static str = PROFILE_FILE_NAME;

    fn prefix(&self) -> Option<String> {
        self.name.clone()
    }

    /// Creates a basic profile.
    fn create_new(name: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        // Check the org directory exists, if not, create it.
        if !Self::org_dir().exists() {
            println!("Creating org directory: {:?}", Self::org_dir());
            std::fs::create_dir(Self::org_dir()).expect("Failed to create org directory.");
        }

        // Check if the app directory exists, if not, create it.
        if !Self::app_dir().exists() {
            println!("Creating app directory: {:?}", Self::app_dir());
            std::fs::create_dir(Self::app_dir()).expect("Failed to create app directory.");
        }

        let profile_file = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };
        // Don't overwrite existing profiles.
        if profile_file.exists() {
            return Ok(Self::load(Some(profile_file))?);
        }

        let mut formatted_path = Self::file_name_ending();
        if let Some(name) = name.clone() {
            formatted_path = format!("{}.{}", name, formatted_path);
        }

        let profile_path = Self::dir().join(formatted_path);
        let file = File::create(profile_path)?;

        let value = UserProfile {
            contacts: Contacts::new(),
            rpcs: RPCList::new(),
            name,
            coins: CoinList::default(),
            portfolio: Portfolio::default(),
            anvil_snapshot: None,
        };

        serde_json::to_writer_pretty(file, &value)?;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_profile_create_new() {
        let result = UserProfile::create_new(Some("test2".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().file_path()).exists());
    }

    #[test]
    fn test_profile_load() {
        let profile = UserProfile::create_new(Some("test".to_string())).unwrap();
        let path_of = profile.file_path();
        let loaded_profile = UserProfile::load(Some(path_of));
        assert!(loaded_profile.is_ok());
    }

    #[test]
    fn test_profile_save() {
        let profile = UserProfile::create_new(Some("test".to_string())).unwrap();
        let save_result = profile.save();
        assert!(save_result.is_ok());
    }

    // This function runs after all tests and cleans up the test profile files
    #[test]
    fn cleanup() {
        let profile = UserProfile::default();
        let path = profile.file_path();
        fs::remove_file(path).unwrap();
    }
}
