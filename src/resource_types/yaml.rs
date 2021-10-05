use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait YamlResourcesExt {
    /// Read yaml file from resources directory and deserialize it.
    fn load_from_yaml_file<T>(&self, yaml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    /// Writes yaml file to a path relative from the resources directory.
    fn save_to_yaml_file<C: ?Sized>(&self, yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
}

impl YamlResourcesExt for Resources {
    /// Read yaml file from resources directory and deserialize it.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the yaml_resources feature in Cargo.toml
    /// use appres::yaml::YamlResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Load and parse the config.yaml file in the assets folder
    /// let config: Config = resources.load_from_yaml_file("config.yaml").unwrap();
    /// ```
    fn load_from_yaml_file<T>(&self, yaml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file_content = self.load_from_file(yaml_file)?;
        Ok(serde_yaml::from_str(&file_content)?)
    }

    /// Writes yaml file to a path relative from the resources directory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the yaml_resources feature in Cargo.toml
    /// use appres::yaml::YamlResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Write config to the config.yaml file in the assets folder
    /// let config = Config { stuff: String::from("Hello World") };
    /// resources.save_to_yaml_file("config.yaml", &config).unwrap();
    /// ```
    fn save_to_yaml_file<C: ?Sized>(&self, yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = serde_yaml::to_vec(&thing)?;
        self.save_to_file(yaml_file, serialized_thing)
    }
}

/// Deserialize a slice in yaml format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the yaml_resources feature in Cargo.toml
/// use appres::yaml::load_yaml_from_slice;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the bytes as a yaml object
/// let config: Config = load_yaml_from_slice(r#"---\nstuff: Hello World"#.as_bytes()).unwrap();
/// ```
pub fn load_yaml_from_slice<T>(yaml_content: impl AsRef<[u8]>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_yaml::from_slice(yaml_content.as_ref())?)
}

/// Deserialize a string in yaml format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the yaml_resources feature in Cargo.toml
/// use appres::yaml::load_yaml_from_str;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the string as a yaml object
/// let config: Config = load_yaml_from_str(r#"---\nstuff: Hello World"#).unwrap();
/// ```
pub fn load_yaml_from_str<T>(yaml_content: impl AsRef<str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_yaml::from_str(yaml_content.as_ref())?)
}

/// Serialize an object into yaml format and write it to a file as specified by the given
/// path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the yaml_resources feature in Cargo.toml
/// use appres::yaml::save_to_yaml_file;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Write the config to config.yaml
/// let config = Config { stuff: String::from("Hello World") };
/// save_to_yaml_file("config.yaml", &config).unwrap();
/// ```
pub fn save_to_yaml_file<C: ?Sized>(yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = serde_yaml::to_vec(&thing)?;

    let yaml_file_dir = yaml_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(yaml_file_dir)?;
    Ok(write(yaml_file, serialized_thing)?)
}
