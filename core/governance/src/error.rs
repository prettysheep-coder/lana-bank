use thiserror::Error;

#[derive(Error, Debug)]
pub enum GovernanceError {
    #[error("GovernanceError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("GovernanceError - AuthorizationError: {0}")]
    AuthorizationError(#[from] authz::error::AuthorizationError),
    #[error("GovernanceError - CommitteeError: {0}")]
    CommitteeError(#[from] super::committee::error::CommitteeError),
    #[error("GovernanceError - PolicyError: {0}")]
    PolicyError(#[from] super::policy::error::PolicyError),
    #[error("GovernanceError - Audit: {0}")]
    AuditError(#[from] audit::error::AuditError),
}
