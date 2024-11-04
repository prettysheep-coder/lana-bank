use thiserror::Error;

use crate::primitives::{UsdCents, WithdrawalId};

#[derive(Error, Debug)]
pub enum WithdrawalError {
    #[error("WithdrawError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("WithdrawError - EsEntityError: {0}")]
    EsEntityError(es_entity::EsEntityError),
    #[error("WithdrawError - LedgerError: {0}")]
    LedgerError(#[from] crate::ledger::error::LedgerError),
    #[error("WithdrawError - GovernanceError: {0}")]
    GovernanceError(#[from] governance::error::GovernanceError),
    #[error("WithdrawError - UserError: {0}")]
    CustomerError(#[from] crate::customer::error::CustomerError),
    #[error("WithdrawError - NotApproved: {0}")]
    NotApproved(WithdrawalId),
    #[error("WithdrawError - AlreadyConfirmed: {0}")]
    AlreadyConfirmed(WithdrawalId),
    #[error("WithdrawError - AlreadyCancelled: {0}")]
    AlreadyCancelled(WithdrawalId),
    #[error("WithdrawError - AuthorizationError: {0}")]
    AuthorizationError(#[from] crate::authorization::error::AuthorizationError),
    #[error("WithdrawError - InsufficientBalance: {0} < {1}")]
    InsufficientBalance(UsdCents, UsdCents),
    #[error("WithdrawError - AuditError: {0}")]
    AuditError(#[from] audit::error::AuditError),
    #[error("WithdrawError - JobError: {0}")]
    JobError(#[from] crate::job::error::JobError),
}

es_entity::from_es_entity_error!(WithdrawalError);
