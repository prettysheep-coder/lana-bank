use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicantError {
    #[error("ApplicantError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("ApplicantError - EntityError: {0}")]
    EntityError(#[from] crate::entity::EntityError),
    #[error("ApplicantError - InvalidPayload: {0}")]
    InvalidPayload(#[from] serde_json::Error),
    #[error("ApplicantError - InvalidUserId: {0}")]
    InvalidUserId(String),
    #[error("ApplicantError - UpdatingEntryError: {0}")]
    UpdatingEntryError(String),
}
