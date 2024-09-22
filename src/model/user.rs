use std::{fs::File, path::PathBuf};

use anyhow::Result;
use directories_next::{self, ProjectDirs};
use serde::{de::DeserializeOwned, Serialize};
use tracing;

use super::{contacts::Contacts, rpcs::RPCList};
use crate::app::AnvilSave;

pub const PROFILE_FILE_EXTENSION: &str = "json";
pub const PROFILE_FILE_NAME: &str = "profile";
pub const QUALIFIER: &str = "xyz.primitive";
pub const ORGANIZATION: &str = "Primitive";
pub const APPLICATION: &str = "Mythic";

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

#[allow(dead_code)]
pub fn get_data_dir() -> PathBuf {
    check_in_executable_dir().unwrap_or_else(|| {
        let dir = directories_next::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .expect("Failed to find valid data directory.");
        dir.data_dir().to_path_buf()
    })
}

fn check_in_executable_dir() -> Option<PathBuf> {
    let base = directories_next::BaseDirs::new()?;
    let dir = base.executable_dir()?;

    dir.join(PROFILE_FILE_NAME)
        .is_file()
        .then(|| dir.to_path_buf())
}

pub trait Saveable: Serialize + DeserializeOwned + Sized {
    const EXTENSION: &'static str = "json";
    const SUFFIX: &'static str;

    fn create_new(name: Option<String>) -> Result<Self>;

    fn org_dir() -> PathBuf {
        Self::app_dir()
            .as_path()
            .parent()
            .expect("Could not get parent directory of app.")
            .to_path_buf()
    }

    fn app_dir() -> PathBuf {
        let dir = get_project_dir().expect("Failed to get project directories.");
        dir.data_dir()
            .parent()
            .expect("Could not get parent of data dir.")
            .to_path_buf()
    }

    fn config_dir() -> PathBuf {
        let dir = get_config_dir();

        if !dir.exists() {
            println!("Creating profile directory: {:?}", dir.as_path());
            std::fs::create_dir_all(dir.as_path()).expect("Failed to create profile directory.");
        }

        dir
    }

    fn dir() -> PathBuf {
        Self::config_dir()
    }

    fn file_name_ending() -> String {
        format!("{}.{}", Self::SUFFIX, Self::EXTENSION)
    }

    fn path() -> PathBuf {
        let formatted = Self::file_name_ending();
        Self::dir().join(formatted)
    }

    fn file_path(&self) -> PathBuf {
        let mut formatted = Self::file_name_ending();

        if let Some(prefix) = self.prefix() {
            formatted = format!("{}.{}", prefix, formatted);
        }
        Self::dir().join(formatted)
    }

    fn file_path_with_name(name: String) -> PathBuf {
        let mut formatted = Self::file_name_ending();
        formatted = format!("{}.{}", name, formatted);
        Self::dir().join(formatted)
    }

    fn prefix(&self) -> Option<String> {
        None
    }

    fn save(&self) -> Result<()> {
        let path = self.file_path();
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = match path {
            Some(path) => path,
            None => Self::path(),
        };

        if !path.exists() {
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
    pub anvil_snapshot: Option<AnvilSave>,
}

impl UserProfile {}

impl Saveable for UserProfile {
    const EXTENSION: &'static str = PROFILE_FILE_EXTENSION;
    const SUFFIX: &'static str = PROFILE_FILE_NAME;

    fn prefix(&self) -> Option<String> {
        self.name.clone()
    }

    fn create_new(name: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        if !Self::org_dir().exists() {
            println!("Creating org directory: {:?}", Self::org_dir());
            std::fs::create_dir(Self::org_dir()).expect("Failed to create org directory.");
        }

        if !Self::app_dir().exists() {
            println!("Creating app directory: {:?}", Self::app_dir());
            std::fs::create_dir(Self::app_dir()).expect("Failed to create app directory.");
        }

        let profile_file = match name.clone() {
            Some(name) => Self::file_path_with_name(name),
            None => Self::path(),
        };
        if profile_file.exists() {
            return Self::load(Some(profile_file));
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

    #[test]
    fn cleanup() {
        let profile = UserProfile::default();
        let path = profile.file_path();
        fs::remove_file(path).unwrap();
    }
}
