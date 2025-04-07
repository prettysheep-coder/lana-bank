mod entity;
pub mod error;
mod ledger;
mod primitives;
mod repo;

use std::num::{NonZeroU8, NonZeroUsize};

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::{CalaLedger, JournalId};
use ledger::ManualTransactionLedger;

use crate::{Chart, LedgerAccountId};

use error::*;
pub use primitives::*;

#[derive(Clone)]
pub struct ManualTransactions<Perms>
where
    Perms: PermissionCheck,
{
    ledger: ManualTransactionLedger,
    authz: Perms,
    journal_id: JournalId,
}

impl<Perms> ManualTransactions<Perms>
where
    Perms: PermissionCheck,
{
    pub fn new(authz: &Perms, cala: &CalaLedger, journal_id: JournalId) -> Self {
        Self {
            ledger: ManualTransactionLedger::new(cala, journal_id),
            authz: authz.clone(),
            journal_id,
        }
    }

    pub async fn execute(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        chart: &Chart,
        description: String,
        entries: Vec<ManualEntryInput>,
    ) -> Result<(), ManualTransactionError> {
        // check how many entries exist
        // lookup template for that amount of entries
        //   If it does not exist yet - create it
        // resolve all account ids to leaf accounts
        //   if its an account code -> lazy create the 'manual' account that backs the COA account
        //   set

        let mut account_ids: Vec<LedgerAccountId> = vec![];

        for i in &entries {
            account_ids.push(
                self.ledger
                    .resolve_account_ref(chart, &i.account_ref)
                    .await?,
            );
        }

        let n = NonZeroU8::try_from(NonZeroUsize::try_from(entries.len()).unwrap()).unwrap();

        let x = self.ledger.create_transaction(n).await?;

        unimplemented!()
    }
}
