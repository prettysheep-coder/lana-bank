mod template;

use cala_ledger::{CalaLedger, JournalId};

use cala_ledger::{account::NewAccount, AccountId, AccountSetId};

use crate::{primitives::AccountCode, Chart, LedgerAccountId};

use super::{
    error::ManualTransactionError,
    primitives::{AccountIdOrCode, CalaTransactionId},
};

use template::*;
pub use template::{EntryParams, ManualTransactionParams};

#[derive(Clone)]
pub struct ManualTransactionLedger {
    cala: CalaLedger,
    journal_id: JournalId,
}

impl ManualTransactionLedger {
    pub fn new(cala: &CalaLedger, journal_id: JournalId) -> Self {
        Self {
            cala: cala.clone(),
            journal_id,
        }
    }

    pub async fn execute(
        &self,
        op: es_entity::DbOp<'_>,
        tx_id: impl Into<CalaTransactionId>,
        params: ManualTransactionParams,
    ) -> Result<(), ManualTransactionError> {
        let tx_id = tx_id.into();
        let mut op = self.cala.ledger_operation_from_db_op(op);

        let _ = ManualTransactionTemplate::init(&self.cala, params.entry_params.len()).await?;

        // self.post_transaction();
        //
        //
        op.commit().await?;
        Ok(())
    }

    pub async fn resolve_account_ref(
        &self,
        chart: &Chart,
        account_ref: &AccountIdOrCode,
    ) -> Result<LedgerAccountId, ManualTransactionError> {
        match account_ref {
            AccountIdOrCode::Id(account_id) => Ok(*account_id),
            AccountIdOrCode::Code(code) => match chart.account_spec(code) {
                Some((_, parent_id)) => {
                    self.find_or_create_manual_account(
                        parent_id,
                        code,
                        code.manual_account_external_id(chart.id),
                    )
                    .await
                }
                None => todo!("err"),
            },
        }
    }

    async fn find_or_create_manual_account(
        &self,
        parent_id: &AccountSetId,
        parent_code: &AccountCode,
        external_id: String,
    ) -> Result<LedgerAccountId, ManualTransactionError> {
        let manual_account = self
            .cala
            .accounts()
            .find_by_external_id(external_id.clone())
            .await;

        match manual_account {
            Ok(existing) => Ok(existing.id().into()),
            Err(e) if e.was_not_found() => {
                self.create_manual_account_set(parent_id, parent_code, &external_id)
                    .await
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn create_manual_account_set(
        &self,
        parent_id: &AccountSetId,
        parent_code: &AccountCode,
        external_id: &str,
    ) -> Result<LedgerAccountId, ManualTransactionError> {
        let manual_account = self
            .cala
            .accounts()
            .create(
                NewAccount::builder()
                    .name(format!("{} Manual", parent_code))
                    .id(AccountId::new())
                    .code(external_id)
                    .external_id(external_id)
                    .build()
                    .unwrap(),
            )
            .await?;

        self.cala
            .account_sets()
            .add_member(*parent_id, manual_account.id)
            .await?;

        Ok(manual_account.id.into())
    }
}
