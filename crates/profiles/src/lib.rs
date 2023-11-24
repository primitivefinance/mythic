pub mod coins;
pub mod profile;
pub mod system;

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait Saveable: Serialize + DeserializeOwned + Sized {
    /// Creates a new instance of this type and writes it to disk with `name`.
    /// todo: be careful about overwriting.
    fn create_new(name: Option<String>) -> Result<Self>;

    /// File extension of the file.
    const EXTENSION: &'static str = "json";
    /// File suffix of the file. e..g `config` for `config.json`.
    const SUFFIX: &'static str;

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
        let dir = system::get_project_dir().expect("Failed to get project directories.");
        dir.data_dir()
            .parent()
            .expect("Could not get parent of data dir.")
            .to_path_buf()
    }

    /// Gets the directory which stores configuration data of the project.
    fn config_dir() -> PathBuf {
        let dir = system::get_config_dir();

        // Handle better directory creation for other systems.
        if !dir.exists() {
            println!("Creating profile directory: {:?}", dir.as_path());
            std::fs::create_dir(dir.as_path()).expect("Failed to create profile directory.");
        }

        dir
    }

    /// Formats the file to have a consistent ending, so it can be validated
    /// when loading.
    fn file_name_ending() -> String {
        format!("{}.{}", Self::SUFFIX, Self::EXTENSION)
    }

    /// Formatted path of these file _types_ in the config directory.
    fn path() -> PathBuf {
        let formatted = Self::file_name_ending();
        Self::config_dir().join(formatted)
    }

    /// Path of the file of this instance.
    fn file_path(&self) -> PathBuf {
        let mut formatted = Self::file_name_ending();

        if let Some(prefix) = self.prefix() {
            formatted = format!("{}.{}", prefix, formatted);
        }

        let path = Self::config_dir().join(formatted);

        println!("Path: {:?}", path);
        path
    }

    fn file_path_with_name(name: String) -> PathBuf {
        let mut formatted = Self::file_name_ending();

        formatted = format!("{}.{}", name, formatted);

        let path = Self::config_dir().join(formatted);

        println!("Path: {:?}", path);
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

        let file = File::open(path)?;

        let instance = serde_json::from_reader(file)?;

        Ok(instance)
    }
}
