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

/// Resources represents a directory where an application would store resources such as
/// config files and other assets.  Makes it easy to load and write resources relative to
/// that directory.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use appres::Resources;
///
/// let resources = Resources::new_app_relative_to_config("projectile").unwrap();
///
/// // Check if the projectile directory has a file named list.yaml.
/// assert!(!resources.has_file("list.yaml"));
///
/// // Check if the projectile directory has a directory named hello_world.
/// assert!(!resources.has_dir("hello_world"));
/// ```
///
/// # Features
///
/// This library also contains extended functionality for json, toml, and yaml files that
/// are feature-gated.
///
/// - **json_resources**: Enabling this feature gives you extra methods (through the
///   `JsonResourcesExt` trait) and extra functions for working with json files.
/// - **toml_resources**: Enabling this feature gives you extra methods (through the
///   `TomlResourcesExt` trait) and extra functions for working with toml files.
/// - **yaml_resources**: Enabling this feature gives you extra methods (through the
///   `YamlResourcesExt` trait) and extra functions for working with yaml files.
///
/// For example, if you enable the `yaml_resources` feature in Cargo.toml...
///
/// ```no_run
/// use appres::Resources;
/// use appres::yaml::YamlResourcesExt;
///
/// // Create a resources manager for the projectile directory in the config directory.
/// let resources = Resources::new_app_relative_to_config("projectile").unwrap();
///
/// // Create a list of strings.
/// let list = vec!["a".to_string(), "b".to_string(), "c".to_string()];
///
/// // Write the serialized list to list.yaml in the projectile directory of the config
/// // directory.
/// resources.save_to_yaml_file("list.yaml", &list);
///
/// // Load the list from the file again.
/// let list_copy: Vec<String> = resources.load_from_yaml_file("list.yaml").unwrap();
/// assert_eq!(list, list_copy);
///
/// // Check for the presence of the list.yaml file
/// assert!(resources.has_file("list.yaml"));
/// ```
#[derive(Clone, Debug)]
pub struct Resources {
    path: PathBuf,
    #[cfg(feature = "toml_resources")]
    resources: Vec<String>,
}

impl Resources {
    /// Creates a resource manager for the given path.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use appres::Resources;
    ///
    /// let root = PathBuf::from("/");
    ///
    /// // Create a new Resources for the root directory.
    /// let resources = Resources::new(root);
    /// ```
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            #[cfg(feature = "toml_resources")]
            resources: vec![],
        }
    }

    /// Creates a resource manager for the config directory.  An error may be returned if
    /// the config path cannot be retrieved.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use appres::Resources;
    ///
    /// // Create a new Resources for the config directory.
    /// let resources = Resources::new_relative_to_config().unwrap();
    /// ```
    pub fn new_relative_to_config() -> Result<Self> {
        let config_dir_path = get_config_path()?;
        Ok(Resources::new(config_dir_path))
    }

    /// Creates a resource manager for the executable directory.  An error may be
    /// returned if the executable path cannot be retrieved.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use appres::Resources;
    ///
    /// // Create a new Resources for the directory the executable resides in.
    /// let resources = Resources::new_relative_to_executable().unwrap();
    /// ```
    pub fn new_relative_to_executable() -> Result<Self> {
        let executable_dir_path = get_executable_dir_path()?;
        Ok(Resources::new(executable_dir_path))
    }

    /// Creates a resource manager for the specified app in the config directory.  An
    /// error may be returned if the config path cannot be retrieved.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use appres::Resources;
    ///
    /// // Create a new Resources for the projectile app in the config directory.
    /// let resources = Resources::new_app_relative_to_config("projectile").unwrap();
    /// ```
    pub fn new_app_relative_to_config(app_name: impl AsRef<str>) -> Result<Self> {
        Resources::new_dir_relative_to_config(app_name.as_ref())
    }

    /// Creates a resource manager for the specified directory in the config directory.
    /// An error may be returned if the config path cannot be retrieved.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use appres::Resources;
    ///
    /// // Create a new Resources for the projectile dir in the config directory.
    /// let resources = Resources::new_dir_relative_to_config("projectile").unwrap();
    /// ```
    pub fn new_dir_relative_to_config(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_config_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    /// Creates a resource manager for the specified directory in the executable
    /// directory.  An error may be returned if the executable path cannot be retrieved.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use appres::Resources;
    ///
    /// // Create a new Resources for the assets dir in the directory that the executable
    /// // resides in.
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// ```
    pub fn new_dir_relative_to_executable(dir: impl AsRef<Path>) -> Result<Self> {
        let mut dir_path = get_executable_dir_path()?;
        dir_path.push(dir);
        Ok(Resources::new(dir_path))
    }

    /// Loads a file at the path specified relative to the directory that was given when
    /// the resource manager was created. Returns a String or an error if the file could
    /// not be accessed for some reason.
    ///
    /// For supported file types, enable the respective feature to load and parse the file.
    /// For example, enable the `toml_resources` feature to access the `TomlResourcesExt`
    /// trait that allows you to load and parse a toml file.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use appres::Resources;
    ///
    /// // Read the config.toml file in the assets folder
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// let config_string = resources.load_from_file("config.toml").unwrap();
    /// ```
    pub fn load_from_file(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        Ok(read_to_string(file_path)?)
    }

    /// Saves a file at the path specified relative to the directory that was given when
    /// the resource manager was created. An error may be returned if the file could not
    /// be written to disk.
    ///
    /// For supported file types, enable the respective feature to serialize the data and
    /// then write to disk. For example, enable the `toml_resources` feature to access the
    /// `TomlResourcesExt` trait that allows you to save an object in toml format.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use appres::Resources;
    ///
    /// let username = "Bob";
    ///
    /// // Save some string to the username file in the assets folder
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// resources.save_to_file("username", username.as_bytes()).unwrap();
    /// ```
    pub fn save_to_file(&self, path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
        let mut file_path = self.path.clone();
        file_path.push(path);

        create_dir_all(&file_path.parent().ok_or(AppResError::NoParent)?)?;
        Ok(write(file_path, content.as_ref())?)
    }

    /// Checks to see if the given path is a regular file that exists relative to the directory that
    /// was given when the resource manager was created.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use appres::Resources;
    ///
    /// // Check if the assets folder contains a config.toml file.
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// assert!(resources.has_file("config.toml"));
    /// ```
    pub fn has_file(&self, path: impl AsRef<Path>) -> bool {
        let mut file_path = self.path.clone();
        file_path.push(path);

        file_path.is_file()
    }

    /// Checks to see if the given path is a directory that exists relative to the directory that was
    /// given when the resource manager was created.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use appres::Resources;
    ///
    /// // Check if the assets folder contains a scripts directory.
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// assert!(resources.has_dir("scripts"));
    /// ```
    pub fn has_dir(&self, path: impl AsRef<Path>) -> bool {
        let mut file_path = self.path.clone();
        file_path.push(path);

        file_path.is_dir()
    }

    /// Returns the full base path for the resource manager.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    ///
    /// use appres::Resources;
    ///
    /// // Assume that the executable resides in /home/nobody and get the path.
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// assert_eq!(PathBuf::from("/home/nobody"), resources.get_path())
    /// ```
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    /// Returns the full path for the given relative path.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    ///
    /// use appres::Resources;
    ///
    /// // Assume that the executable resides in /home/nobody and get the assets path.
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    /// assert_eq!(PathBuf::from("/home/nobody/assets"), resources.get_path())
    /// ```
    pub fn get_file_path(&self, path: impl AsRef<Path>) -> PathBuf {
        let mut file_path = self.path.clone();
        file_path.push(path);
        file_path
    }
}

/// Returns either the executable directory or [`AppResError::IOError`].
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use appres::get_executable_dir_path;
///
/// // Assume that the executable lies in /home/nobody
/// let executable_path = get_executable_dir_path().unwrap();
/// assert_eq!(executable_path, PathBuf::from("/home/nobody"));
/// ```
pub fn get_executable_dir_path() -> Result<PathBuf> {
    let mut executable_dir_path = std::env::current_exe()?;
    executable_dir_path.pop();
    Ok(executable_dir_path)
}

/// Returns either the config directory or [`AppResError::ConfigDirNotFound`].
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use appres::get_config_path;
///
/// // Assume that the config directory is /home/nobody/.config
/// let config_path = get_config_path().unwrap();
/// assert_eq!(config_path, PathBuf::from("/home/nobody/.config"));
/// ```
pub fn get_config_path() -> Result<PathBuf> {
    Option::ok_or(config_dir(), AppResError::ConfigDirNotFound)
}

/// Expands `path` relative to the path of the executable. Returns either the expanded path or
/// [`AppResError::IOError`].
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use appres::get_file_path_relative_to_executable_path;
///
/// // Assume that the executable lies in /home/nobody
/// let assets_path = get_file_path_relative_to_executable_path("assets").unwrap();
/// assert_eq!(assets_path, PathBuf::from("/home/nobody/assets"));
/// ```
pub fn get_file_path_relative_to_executable_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let mut file_path = get_executable_dir_path()?;
    file_path.push(path);
    Ok(file_path)
}

/// Expands `path` relative to the path of the config dir. Returns either the expanded path or
/// [`AppResError::ConfigDirNotFound`].
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use appres::get_file_path_relative_to_config_path;
///
/// // Assume that the executable lies in /home/nobody/.config
/// let projects_path = get_file_path_relative_to_config_path("projectile/projects.yaml")
///     .unwrap();
/// assert_eq!(projects_path, PathBuf::from("/home/nobody/.config/projectile/projects.yaml"));
/// ```
pub fn get_file_path_relative_to_config_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let mut file_path = get_config_path()?;
    file_path.push(path);
    Ok(file_path)
}

/// Read the content of file given its path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use appres::read_from_file;
///
/// // Read config.toml into a string
/// let content = read_from_file("config.toml").unwrap();
/// ```
pub fn read_from_file(path: impl AsRef<Path>) -> Result<String> {
    Ok(read_to_string(path)?)
}

/// Writes a slice to a file specified by the given path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use appres::save_slice_to_file;
///
/// // Write a slice of bytes to config.toml
/// save_slice_to_file("config.toml", "Hello World".as_bytes()).unwrap();
/// ```
pub fn save_slice_to_file(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();

    create_dir_all(path.parent().ok_or(AppResError::NoParent)?)?;
    Ok(write(path, content.as_ref())?)
}

/// Writes a str to a file specified by the given path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use appres::save_str_to_file;
///
/// // Write a string to config.toml
/// save_str_to_file("config.toml", "Hello World").unwrap();
/// ```
pub fn save_str_to_file(path: impl AsRef<Path>, content: impl AsRef<str>) -> Result<()> {
    save_slice_to_file(path, content.as_ref().as_bytes())
}
