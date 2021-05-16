use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait TomlResourcesExt {
    fn load_from_toml_file<'de, T>(&'de mut self, toml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::Deserialize<'de>;
    fn save_to_toml_file<C: ?Sized>(&self, toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
}

impl TomlResourcesExt for Resources {
    fn load_from_toml_file<'de, T>(&'de mut self, toml_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        let file_content = self.load_from_file(toml_file)?;
        self.resources.push(file_content);

        let resource = &self.resources[self.resources.len() - 1];
        Ok(toml::from_str(resource)?)
    }

    fn save_to_toml_file<C: ?Sized>(&self, toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize,
    {
        let serialized_thing = toml::to_vec(&thing)?;
        self.save_to_file(toml_file, serialized_thing)
    }
}

pub fn load_toml_from_slice<'de, T>(toml_content: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    Ok(toml::from_slice(toml_content)?)
}

pub fn load_toml_from_str<'de, T>(toml_content: &'de str) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    Ok(toml::from_str(toml_content)?)
}

pub fn save_to_toml_file<C: ?Sized>(toml_file: impl AsRef<Path>, thing: &C) -> Result<()>
where
    C: serde::Serialize,
{
    let serialized_thing = toml::to_vec(&thing)?;

    let toml_file_dir = toml_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(toml_file_dir)?;
    Ok(write(toml_file, serialized_thing)?)
}
