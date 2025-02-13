use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustodianError {
    #[error("CustodianError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("CustodianError - EsEntityError: {0}")]
    EsEntityError(es_entity::EsEntityError),
    #[error("CustodianError - CursorDestructureError: {0}")]
    CursorDestructureError(#[from] es_entity::CursorDestructureError),
    #[error("CustodianError - AuthorizationError: {0}")]
    AuthorizationError(#[from] authz::error::AuthorizationError),
    #[error("CustodianError - AuditError: ${0}")]
    AuditError(#[from] audit::error::AuditError),
}

es_entity::from_es_entity_error!(CustodianError);
