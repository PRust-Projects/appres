use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait JsonResourcesExt {
    /// Read json file from resources directory and deserialize it.
    fn load_from_json_file<T>(&self, json_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    /// Write json file to a path relative from the resources directory.
    fn save_to_json_file<C: ?Sized>(&self, json_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
    /// Write json file to a path relative from the resources directory in a pretty format.
    fn pretty_save_to_json_file<C: ?Sized>(
        &self,
        json_file: impl AsRef<Path>,
        thing: &C,
    ) -> Result<()>
    where
        C: serde::Serialize;
}

impl JsonResourcesExt for Resources {
    /// Read json file from resources directory and deserialize it.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the json_resources feature in Cargo.toml
    /// use appres::json::JsonResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Load and parse the config.json file in the assets folder
    /// let config: Config = resources.load_from_json_file("config.json").unwrap();
    /// ```
    fn load_from_json_file<T>(&self, json_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file_content = self.load_from_file(json_file)?;
        Ok(serde_json::from_str(&file_content)?)
    }

    /// Write json file to a path relative from the resources directory.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the json_resources feature in Cargo.toml
    /// use appres::json::JsonResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Write config to the config.json file in the assets folder
    /// let config = Config { stuff: String::from("Hello World") };
    /// resources.save_to_json_file("config.json", &config).unwrap();
    /// ```
    fn save_to_json_file<C: ?Sized>(&self, json_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = serde_json::to_vec(&thing)?;
        self.save_to_file(json_file, serialized_thing)
    }

    /// Write json file to a path relative from the resources directory in a pretty format.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use serde::{Deserialize, Serialize};
    ///
    /// use appres::Resources;
    /// // Note you need to enable the json_resources feature in Cargo.toml
    /// use appres::json::JsonResourcesExt;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct Config {
    ///     stuff: String,
    /// }
    ///
    /// let resources = Resources::new_dir_relative_to_executable("assets").unwrap();
    ///
    /// // Write config to the config.json file in the assets folder (in pretty json format)
    /// let config = Config { stuff: String::from("Hello World") };
    /// resources.pretty_save_to_json_file("config.json", &config).unwrap();
    /// ```
    fn pretty_save_to_json_file<C: ?Sized>(
        &self,
        json_file: impl AsRef<Path>,
        thing: &C,
    ) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = serde_json::to_vec_pretty(&thing)?;
        self.save_to_file(json_file, serialized_thing)
    }
}

/// Deserialize a slice in json format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the json_resources feature in Cargo.toml
/// use appres::json::load_json_from_slice;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the bytes as a json object
/// let config: Config = load_json_from_slice(r#"{"stuff": "Hello World"}"#.as_bytes()).unwrap();
/// ```
pub fn load_json_from_slice<T>(json_content: impl AsRef<[u8]>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_slice(json_content.as_ref())?)
}

/// Deserialize a string in json format.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the json_resources feature in Cargo.toml
/// use appres::json::load_json_from_str;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Parse the string as a json object
/// let config: Config = load_json_from_str(r#"{"stuff": "Hello World"}"#).unwrap();
/// ```
pub fn load_json_from_str<T>(json_content: impl AsRef<str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_str(json_content.as_ref())?)
}

/// Serialize an object into json format and write it to a file as specified by the given path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the json_resources feature in Cargo.toml
/// use appres::json::save_to_json_file;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Write the config to config.json
/// let config = Config { stuff: String::from("Hello World") };
/// save_to_json_file("config.json", &config).unwrap();
/// ```
pub fn save_to_json_file<C: ?Sized>(json_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = serde_json::to_vec(&thing)?;

    let json_file_dir = json_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(json_file_dir)?;
    Ok(write(json_file, serialized_thing)?)
}

/// Serialize an object into json pretty format and write it to a file as specified by the given
/// path.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use serde::{Deserialize, Serialize};
///
/// // Note that you need to enable the json_resources feature in Cargo.toml
/// use appres::json::pretty_save_to_json_file;
///
/// #[derive(Deserialize, Serialize)]
/// struct Config {
///     stuff: String,
/// }
///
/// // Write the config to config.json
/// let config = Config { stuff: String::from("Hello World") };
/// pretty_save_to_json_file("config.json", &config).unwrap();
/// ```
pub fn pretty_save_to_json_file<C: ?Sized>(json_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = serde_json::to_vec_pretty(&thing)?;

    let json_file_dir = json_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(json_file_dir)?;
    Ok(write(json_file, serialized_thing)?)
}
