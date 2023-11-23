//! Save and load profiles from disk.

use std::{fs::File, path::PathBuf};

use api::contacts::Contacts;

use super::system::{self, PROFILE_FILE_NAME};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub data_dir: PathBuf,
    pub contacts: Contacts,
    pub name: Option<String>,
}

impl Profile {
    pub fn app_dir() -> PathBuf {
        let dir = system::get_project_dir().expect("Failed to get project directories.");
        dir.data_dir()
            .parent()
            .expect("Could not get parent of data dir.")
            .to_path_buf()
    }

    pub fn org_dir() -> PathBuf {
        Self::app_dir()
            .as_path()
            .parent()
            .expect("Could not get parent directory of app.")
            .to_path_buf()
    }

    /// Gets the directory which stores profiles.
    pub fn profile_dir() -> PathBuf {
        let dir = system::get_config_dir();

        // Handle better directory creation for other systems.
        if !dir.exists() {
            println!("Creating profile directory: {:?}", dir.as_path());
            std::fs::create_dir(dir.as_path()).expect("Failed to create profile directory.");
        }

        dir
    }

    fn path() -> PathBuf {
        Self::profile_dir().join(system::PROFILE_FILE_NAME)
    }

    pub fn path_of(&self) -> PathBuf {
        let mut formatted = PROFILE_FILE_NAME.to_string();
        if let Some(name) = &self.name {
            formatted = format!("{}.{}", name, formatted);
        }

        let path = Self::profile_dir().join(formatted);

        println!("Profile path: {:?}", path);
        path
    }

    /// Loads the profile from disk.
    pub fn load(path: Option<PathBuf>) -> anyhow::Result<Self, anyhow::Error> {
        let path = match path {
            Some(path) => path,
            None => Self::path(),
        };

        let file = File::open(path)?;

        let Profile {
            data_dir,
            contacts,
            name,
        } = serde_json::from_reader(file)?;

        Ok(Self {
            data_dir,
            contacts,
            name,
        })
    }

    /// Creates a basic profile.
    pub fn create_new(name: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
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

        let profile_file = Self::path();
        // Don't overwrite existing profiles.
        if profile_file.exists() {
            return Ok(Self::load(Some(profile_file))?);
        }

        let mut formatted_path = PROFILE_FILE_NAME.to_string();
        if let Some(name) = name.clone() {
            formatted_path = format!("{}.{}", name, formatted_path);
        }

        let profile_path = Self::profile_dir().join(formatted_path);
        println!("Creating profile: {:?}", profile_path);
        let file = File::create(profile_path)?;

        let value = Profile {
            data_dir: system::get_data_dir(),
            contacts: Contacts::new(),
            name,
        };

        serde_json::to_writer_pretty(file, &value)?;

        Ok(value)
    }

    /// Saves the profile to disk.
    pub fn save(&self) -> anyhow::Result<(), anyhow::Error> {
        let path = Self::path();
        let file = File::create(path)?;

        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_profile_create_new() {
        let result = Profile::create_new(Some("test".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().path_of()).exists());
    }

    #[test]
    fn test_profile_load() {
        let profile = Profile::create_new(Some("test".to_string())).unwrap();
        let path_of = profile.path_of();
        let loaded_profile = Profile::load(Some(path_of));
        assert!(loaded_profile.is_ok());
    }

    #[test]
    fn test_profile_save() {
        let profile = Profile::create_new(Some("test".to_string())).unwrap();
        let save_result = profile.save();
        assert!(save_result.is_ok());
    }

    // This function runs after all tests and cleans up the test profile files
    #[test]
    fn cleanup() {
        let profile = Profile::default();
        let path = profile.path_of();
        fs::remove_file(path).unwrap();
    }
}
