use std::fs::{create_dir_all, write};
use std::path::Path;

use crate::{AppResError, Resources, Result};

pub trait JsonResourcesExt {
    fn load_from_json_file<T>(&self, json_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    fn save_to_json_file<C: ?Sized>(&self, json_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize;
}

impl JsonResourcesExt for Resources {
    fn load_from_json_file<T>(&self, json_file: impl AsRef<Path>) -> Result<T>
    where
        T: serde::de::DeserializeOwned {
        let file_content = self.load_from_file(json_file)?;
        Ok(serde_json::from_str(&file_content)?)
    }

    fn save_to_json_file<C: ?Sized>(&self, json_file: impl AsRef<Path>, thing: &C) -> Result<()>
    where
        C: serde::Serialize {
        let serialized_thing = serde_json::to_vec(&thing)?;
        self.save_to_file(json_file, serialized_thing)
    }
}

pub fn load_json_from_slice<T>(json_content: impl AsRef<[u8]>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_slice(json_content.as_ref())?)
}

pub fn load_json_from_str<T>(json_content: impl AsRef<str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_str(json_content.as_ref())?)
}

pub fn save_to_json_file<C: ?Sized>(json_file: impl AsRef<Path>, thing: &C) -> Result<()>
where C: serde::Serialize {
    let serialized_thing = serde_json::to_vec(&thing)?;

    let json_file_dir = json_file.as_ref().parent().ok_or(AppResError::NoParent)?;
    create_dir_all(json_file_dir)?;
    Ok(write(json_file, serialized_thing)?)
}
