mod error;
mod resource_types;

use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};

use dirs::config_dir;

pub use error::AppResError;
pub use resource_types::toml;

pub type Result<T> = std::result::Result<T, AppResError>;

pub struct Resources {
    path: PathBuf,
    resources: Vec<String>,
}

impl Resources {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            resources: vec![],
        }
    }

    pub fn new_relative_to_config() -> Result<Self> {
        let config_dir_path = get_config_path()?;
        Ok(Resources::new(config_dir_path))
    }

    pub fn new_relative_to_executable() -> Result<Self> {
        let executable_dir_path = get_executable_dir_path()?;
        Ok(Resources::new(executable_dir_path))
    }

    pub fn new_app_relative_to_config(app_name: impl AsRef<str>) -> Result<Self> {
        Resources::new_dir_relative_to_config(app_name.as_ref())
    }

    pub fn new_dir_relative_to_config(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_config_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    pub fn new_dir_relative_to_executable(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_executable_dir_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    pub fn load_from_file(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(read_to_string(file_path)?)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
        create_dir_all(&self.path)?;

        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(write(file_path, content.as_ref())?)
    }
}

pub fn get_executable_dir_path() -> Result<PathBuf> {
    let mut executable_dir_path = std::env::current_exe()?;
    executable_dir_path.pop();
    Ok(executable_dir_path)
}

pub fn get_config_path() -> Result<PathBuf> {
    Option::ok_or(config_dir(), AppResError::ConfigDirNotFound)
}
