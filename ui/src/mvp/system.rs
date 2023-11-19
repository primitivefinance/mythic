use std::{env, path::PathBuf};

use directories_next::{self, ProjectDirs};

pub const PROFILE_FILE_NAME: &str = "profile.json";
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
