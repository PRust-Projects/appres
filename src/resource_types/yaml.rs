use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait YamlResourcesExt {
    /// Read yaml file from resources directory and deserialize it.
    fn load_from_yaml_file<T>(&self, yaml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    /// Write yaml file to a path relative from the resources directory.
    fn save_to_yaml_file<C: ?Sized>(&self, yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
}

impl YamlResourcesExt for Resources {
    /// Read yaml file from resources directory and deserialize it.
    fn load_from_yaml_file<T>(&self, yaml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file_content = self.load_from_file(yaml_file)?;
        Ok(serde_yaml::from_str(&file_content)?)
    }

    /// Write yaml file to a path relative from the resources directory.
    fn save_to_yaml_file<C: ?Sized>(&self, yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = serde_yaml::to_vec(&thing)?;
        self.save_to_file(yaml_file, serialized_thing)
    }
}

/// Deserialize a slice in yaml format.
pub fn load_yaml_from_slice<T>(yaml_content: impl AsRef<[u8]>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_yaml::from_slice(yaml_content.as_ref())?)
}

/// Deserialize a string in yaml format.
pub fn load_yaml_from_str<T>(yaml_content: impl AsRef<str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_yaml::from_str(yaml_content.as_ref())?)
}

/// Serialize an object into yaml format and write it to a file as specified by the given
/// path.
pub fn save_to_yaml_file<C: ?Sized>(yaml_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = serde_yaml::to_vec(&thing)?;

    let yaml_file_dir = yaml_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(yaml_file_dir)?;
    Ok(write(yaml_file, serialized_thing)?)
}
