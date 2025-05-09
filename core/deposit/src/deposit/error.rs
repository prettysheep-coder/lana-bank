use thiserror::Error;

#[derive(Error, Debug)]
pub enum DepositError {
    #[error("DepositError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("DepositError - EsEntityError: {0}")]
    EsEntityError(es_entity::EsEntityError),
    #[error("DepositError - CursorDestructureError: {0}")]
    CursorDestructureError(#[from] es_entity::CursorDestructureError),
}

es_entity::from_es_entity_error!(DepositError);
