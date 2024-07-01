use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermError {
    #[error("TermError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("TermError - EntityError: {0}")]
    EntityError(#[from] crate::entity::EntityError),
}
