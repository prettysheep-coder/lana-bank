use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("ExportError - CalaError: {0}")]
    CalaError(#[from] super::cala::error::CalaError),
}
