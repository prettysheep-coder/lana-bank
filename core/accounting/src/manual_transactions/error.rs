use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManualTransactionError {
    #[error("ManualTransactionError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("ManualTransactionError - EsEntityError: {0}")]
    EsEntityError(es_entity::EsEntityError),
    #[error("ManualTransactionError - CursorDestructureError: {0}")]
    CursorDestructureError(#[from] es_entity::CursorDestructureError),
    #[error("ManualTransactionError - TxTemplateError: {0}")]
    TxTemplateError(#[from] cala_ledger::tx_template::error::TxTemplateError),
}

es_entity::from_es_entity_error!(ManualTransactionError);
