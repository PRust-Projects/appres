use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait TomlResourcesExt {
    /// Read toml file from resources directory and deserialize it.
    ///
    /// Note that the content of the toml file is stored in memory for the duration of
    /// the [`Resources`] object due to a limitation in the toml library.  Use
    /// [`load_toml_from_slice`] or [`load_toml_from_str`] if you only need the toml for
    /// a short period of time.
    fn load_from_toml_file<'de, T>(&'de mut self, toml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::Deserialize<'de>;
    /// Write toml file to a path relative from the resources directory.
    fn save_to_toml_file<C: ?Sized>(&self, toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
}

impl TomlResourcesExt for Resources {
    /// Read toml file from resources directory and deserialize it.
    ///
    /// Note that the content of the toml file is stored in memory for the duration of
    /// the [`Resources`] object due to a limitation in the toml library.  Use
    /// [`load_toml_from_slice`] or [`load_toml_from_str`] if you only need the toml for
    /// a short period of time.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the toml_resources feature in Cargo.toml
    /// use appres::toml::TomlResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let mut resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Load and parse the config.toml file in the assets folder
    /// let config: Config = resources.load_from_toml_file("config.toml").unwrap();
    /// ```
    fn load_from_toml_file<'de, T>(&'de mut self, toml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        let file_content = self.load_from_file(toml_file)?;
        self.resources.push(file_content);

        let resource = &self.resources[self.resources.len() - 1];
        Ok(toml::from_str(resource)?)
    }

    /// Write toml file to a path relative from the resources directory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the toml_resources feature in Cargo.toml
    /// use appres::toml::TomlResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Write config to the config.toml file in the assets folder
    /// let config = Config { stuff: String::from("Hello World") };
    /// resources.save_to_toml_file("config.toml", &config).unwrap();
    /// ```
    fn save_to_toml_file<C: ?Sized>(&self, toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = toml::to_vec(&thing)?;
        self.save_to_file(toml_file, serialized_thing)
    }
}

/// Deserialize a slice in toml format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the toml_resources feature in Cargo.toml
/// use appres::toml::load_toml_from_slice;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the bytes as a toml object
/// let config: Config = load_toml_from_slice(r#"stuff = 'Hello World'"#.as_bytes()).unwrap();
/// ```
pub fn load_toml_from_slice<'de, T>(toml_content: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    Ok(toml::from_slice(toml_content)?)
}

/// Deserialize a string in toml format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the toml_resources feature in Cargo.toml
/// use appres::toml::load_toml_from_str;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the string as a toml object
/// let config: Config = load_toml_from_str(r#"stuff = 'Hello World'"#).unwrap();
/// ```
pub fn load_toml_from_str<'de, T>(toml_content: &'de str) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    Ok(toml::from_str(toml_content)?)
}

/// Serialize an object into toml format and write it to a file as specified by the given
/// path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the toml_resources feature in Cargo.toml
/// use appres::toml::save_to_toml_file;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Write the config to config.toml
/// let config = Config { stuff: String::from("Hello World") };
/// save_to_toml_file("config.toml", &config).unwrap();
/// ```
pub fn save_to_toml_file<C: ?Sized>(toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = toml::to_vec(&thing)?;

    let toml_file_dir = toml_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(toml_file_dir)?;
    Ok(write(toml_file, serialized_thing)?)
}
