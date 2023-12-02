//! Save and load profiles from disk.
use std::{fs::File, path::PathBuf};

use super::{
    contacts::Contacts,
    system::{get_data_dir, PROFILE_FILE_EXTENSION, PROFILE_FILE_NAME},
    Saveable,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub data_dir: PathBuf,
    pub contacts: Contacts,
    pub name: Option<String>,
}

impl Saveable for Profile {
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

        let profile_file = Self::path();
        // Don't overwrite existing profiles.
        if profile_file.exists() {
            return Ok(Self::load(Some(profile_file))?);
        }

        let mut formatted_path = Self::file_name_ending();
        if let Some(name) = name.clone() {
            formatted_path = format!("{}.{}", name, formatted_path);
        }

        let profile_path = Self::dir().join(formatted_path);
        println!("Creating profile: {:?}", profile_path);
        let file = File::create(profile_path)?;

        let value = Profile {
            data_dir: get_data_dir(),
            contacts: Contacts::new(),
            name,
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
        let result = Profile::create_new(Some("test".to_string()));
        assert!(result.is_ok());
        assert!(Path::new(&result.unwrap().file_path()).exists());
    }

    #[test]
    fn test_profile_load() {
        let profile = Profile::create_new(Some("test".to_string())).unwrap();
        let path_of = profile.file_path();
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
        let path = profile.file_path();
        fs::remove_file(path).unwrap();
    }
}
