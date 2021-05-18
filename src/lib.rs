mod error;
mod resource_types;

use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};

use dirs::config_dir;

pub use error::AppResError;
#[cfg(feature = "json_resources")]
pub use resource_types::json;
#[cfg(feature = "toml_resources")]
pub use resource_types::toml;
#[cfg(feature = "yaml_resources")]
pub use resource_types::yaml;

/// A shorthand for when the error is of type [`AppResError`].
pub type Result<T> = std::result::Result<T, AppResError>;

/// Represents a directory where an application would store resources such as config
/// files and other assets.  Makes it easy to load and write resources relative to that
/// directory.
pub struct Resources {
    path: PathBuf,
    #[cfg(feature = "toml_resources")]
    resources: Vec<String>,
}

impl Resources {
    /// Create a resource manager for the given path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            #[cfg(feature = "toml_resources")]
            resources: vec![],
        }
    }

    /// Create a resource manager for the config directory.  An error may be returned if
    /// the config path cannot be retrieved.
    pub fn new_relative_to_config() -> Result<Self> {
        let config_dir_path = get_config_path()?;
        Ok(Resources::new(config_dir_path))
    }

    /// Create a resource manager for the executable directory.  An error may be
    /// returned if the executable path cannot be retrieved.
    pub fn new_relative_to_executable() -> Result<Self> {
        let executable_dir_path = get_executable_dir_path()?;
        Ok(Resources::new(executable_dir_path))
    }

    /// Create a resource manager for the specified app in the config directory.  An
    /// error may be returned if the config path cannot be retrieved.
    pub fn new_app_relative_to_config(app_name: impl AsRef<str>) -> Result<Self> {
        Resources::new_dir_relative_to_config(app_name.as_ref())
    }

    /// Create a resource manager for the specified directory in the config directory.
    /// An error may be returned if the config path cannot be retrieved.
    pub fn new_dir_relative_to_config(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_config_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    /// Create a resource manager for the specified directory in the executable
    /// directory.  An error may be returned if the executable path cannot be retrieved.
    pub fn new_dir_relative_to_executable(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_executable_dir_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    /// Load a file at the path specified relative to the directory that was given when
    /// the resource manager was created.
    pub fn load_from_file(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(read_to_string(file_path)?)
    }

    /// Save a file at the path specified relative to the directory that was given when
    /// the resource manager was created.
    pub fn save_to_file(&self, path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
        create_dir_all(&self.path)?;

        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(write(file_path, content.as_ref())?)
    }
}

/// Return either the executable directory or [`AppResError::IOError`].
pub fn get_executable_dir_path() -> Result<PathBuf> {
    let mut executable_dir_path = std::env::current_exe()?;
    executable_dir_path.pop();
    Ok(executable_dir_path)
}

/// Return either the config directory or [`AppResError::ConfigDirNotFound`].
pub fn get_config_path() -> Result<PathBuf> {
    Option::ok_or(config_dir(), AppResError::ConfigDirNotFound)
}
