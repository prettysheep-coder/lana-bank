use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("AuditError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("AuditError - TryFromIntError: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
}
