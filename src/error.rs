use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppResError {
    #[error("cannot find config dir")]
    ConfigDirNotFound,
    #[error(transparent)]
    InvalidTomlDeserialization(#[from] toml::de::Error),
    #[error(transparent)]
    InvalidTomlSerialization(#[from] toml::ser::Error),
    #[error(transparent)]
    InvalidYaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("there is no parent for this directory")]
    NoParent,
}
