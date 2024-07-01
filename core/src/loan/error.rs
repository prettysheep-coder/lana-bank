use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoanError {
    #[error("LoanError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("LoanError - EntityError: {0}")]
    EntityError(#[from] crate::entity::EntityError),
    #[error("LoanError - TermError: {0}")]
    TermError(#[from] super::terms::error::TermError),
}
