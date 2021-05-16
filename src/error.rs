use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppResError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
