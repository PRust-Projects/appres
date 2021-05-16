use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppResError {
    #[error("cannot find config dir")]
    ConfigDirNotFound,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
