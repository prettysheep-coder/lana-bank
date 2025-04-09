pub mod error;
mod value;

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::CalaLedger;
use tracing::instrument;

use crate::primitives::{CoreAccountingAction, CoreAccountingObject, LedgerTransactionId};

use error::*;
pub use value::*;

#[derive(Clone)]
pub struct LedgerTransactions<Perms>
where
    Perms: PermissionCheck,
{
    authz: Perms,
    cala: CalaLedger,
}

impl<Perms> LedgerTransactions<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    pub fn new(authz: &Perms, cala: &CalaLedger) -> Self {
        Self {
            authz: authz.clone(),
            cala: cala.clone(),
        }
    }

    #[instrument(name = "accounting.ledger_transaction.find_by_id", skip(self), err)]
    pub async fn find_by_id(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        id: impl Into<LedgerTransactionId> + std::fmt::Debug,
    ) -> Result<Option<LedgerTransaction>, LedgerTransactionError> {
        let id = id.into();
        self.authz
            .enforce_permission(
                sub,
                CoreAccountingObject::ledger_transaction(id),
                CoreAccountingAction::LEDGER_TRANSACTION_READ,
            )
            .await?;

        let (transaction, entries) = tokio::join!(
            self.cala.transactions().find_by_id(id),
            self.cala.entries().list_for_transaction_id(id)
        );
        let res = match transaction {
            Ok(tx) => Some(LedgerTransaction::try_from((tx, entries?))?),
            Err(e) if e.was_not_found() => None,
            Err(e) => return Err(e.into()),
        };
        Ok(res)
    }
}
