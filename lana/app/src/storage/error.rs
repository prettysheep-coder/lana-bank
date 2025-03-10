use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("StorageError - Utf8Error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}
