mod templates;

use std::num::NonZeroU8;

use cala_ledger::{CalaLedger, JournalId, transaction::Transaction};
use templates::ManualTransactionTemplates;

use crate::{AccountCode, Chart, LedgerAccountId, chart_of_accounts_error::ChartOfAccountsError};

use super::{AccountRef, error::ManualTransactionError};

#[derive(Clone)]
pub struct ManualTransactionLedger {
    cala: CalaLedger,
    templates: ManualTransactionTemplates,
    journal_id: JournalId,
}

impl ManualTransactionLedger {
    pub fn new(cala: &CalaLedger, journal_id: JournalId) -> Self {
        Self {
            cala: cala.clone(),
            templates: ManualTransactionTemplates::new(cala.tx_templates()),
            journal_id,
        }
    }

    pub async fn create_transaction(
        &self,
        n: NonZeroU8,
    ) -> Result<Transaction, ManualTransactionError> {
        let _template = self.templates.get_template_for_n_entries(n).await?;

        Ok(todo!())
    }

    pub async fn resolve_account_ref(
        &self,
        chart: &Chart,
        account_ref: &AccountRef,
    ) -> Result<LedgerAccountId, ManualTransactionError> {
        match account_ref {
            AccountRef::Id(account_id) => Ok(*account_id),
            AccountRef::Code(code) => match chart.account_spec(code) {
                Some((_, id)) => Ok((*id).into()),
                None => self.create_manual_account_set(code).await,
            },
        }
    }

    async fn create_manual_account_set(
        &self,
        code: &AccountCode,
    ) -> Result<LedgerAccountId, ManualTransactionError> {
        todo!()
    }
}
