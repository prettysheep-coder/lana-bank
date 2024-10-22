use thiserror::Error;

#[derive(Error, Debug)]
pub enum GovernanceError {
    #[error("GovernanceError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("GovernanceError - AuthorizationError: {0}")]
    AuthorizationError(#[from] crate::authorization::error::AuthorizationError),
    #[error("GovernanceError - CommitteeError: {0}")]
    CommitteeError(#[from] super::committee::error::CommitteeError),
}
