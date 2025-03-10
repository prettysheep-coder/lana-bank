use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Cloud Storage Error: {0}")]
    CloudStorage(#[from] google_cloud_storage::http::Error),
    #[error("Signed Url Error: {0}")]
    SignedUrl(#[from] google_cloud_storage::sign::SignedURLError),
}
