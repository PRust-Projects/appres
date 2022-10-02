use thiserror::Error;

/// Grouping all errors together to simplify error handling.
#[derive(Debug, Error)]
pub enum AppResError {
    /// Could not find the config directory.
    #[error("cannot find config dir")]
    ConfigDirNotFound,
    /// Could not parse the json when serializing or deserializing.
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    /// Could not parse the toml when deserializing.
    #[error(transparent)]
    InvalidTomlDeserialization(#[from] toml::de::Error),
    /// Could not parse the toml when serializing.
    #[error(transparent)]
    InvalidTomlSerialization(#[from] toml::ser::Error),
    /// Could not parse the yaml when serializing or deserializing.
    #[error(transparent)]
    InvalidYaml(#[from] serde_yaml::Error),
    /// Could not read, write, or access files or directories on the filesystem.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Unable to retrieve the parent for a directory.
    #[error("there is no parent for this directory")]
    NoParent,
}
