mod entity;
pub mod error;
mod ledger;
mod primitives;
mod repo;

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::{CalaLedger, JournalId};
use ledger::{EntryParams, ManualTransactionLedger, ManualTransactionParams};

use crate::{
    primitives::{CoreAccountingAction, CoreAccountingObject},
    Chart, LedgerAccountId,
};

use error::*;
pub use primitives::*;
use repo::*;

#[derive(Clone)]
pub struct ManualTransactions<Perms>
where
    Perms: PermissionCheck,
{
    ledger: ManualTransactionLedger,
    authz: Perms,
    journal_id: JournalId,
    repo: ManualTransactionRepo,
}

impl<Perms> ManualTransactions<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    pub fn new(
        pool: &sqlx::PgPool,
        authz: &Perms,
        cala: &CalaLedger,
        journal_id: JournalId,
    ) -> Self {
        let repo = ManualTransactionRepo::new(pool);
        Self {
            ledger: ManualTransactionLedger::new(cala, journal_id),
            authz: authz.clone(),
            journal_id,
            repo,
        }
    }

    pub async fn execute(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        chart: &Chart,
        description: String,
        entries: Vec<ManualEntryInput>,
    ) -> Result<(), ManualTransactionError> {
        let _audit_info = self
            .authz
            .enforce_permission(
                sub,
                CoreAccountingObject::all_manual_transactions(),
                CoreAccountingAction::MANUAL_TRANSACTION_CREATE,
            )
            .await?;

        let mut account_ids: Vec<LedgerAccountId> = vec![];

        for i in &entries {
            account_ids.push(
                self.ledger
                    .resolve_account_ref(chart, &i.account_ref)
                    .await?,
            );
        }

        let tx_id = CalaTransactionId::new();
        self.ledger
            .execute(
                tx_id,
                ManualTransactionParams {
                    journal_id: self.journal_id,
                    description,
                    entry_params: entries
                        .into_iter()
                        .map(|e| EntryParams {
                            account_id: cala_ledger::AccountId::new(),
                            amount: e.amount,
                            currency: e.currency,
                            direction: e.direction,
                            description: e.description,
                        })
                        .collect(),
                },
            )
            .await?;

        Ok(())
    }
}
