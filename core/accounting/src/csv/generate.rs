use csv::Writer;
use std::io::Cursor;

use crate::journal::JournalEntry;
use crate::ledger_account::LedgerAccounts;
use crate::primitives::LedgerAccountId;
use crate::{CoreAccountingAction, CoreAccountingObject};
use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::DebitOrCredit;

use super::error::AccountingCsvError;

pub struct GenerateCsv<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    ledger_accounts: LedgerAccounts<Perms>,
}

impl<Perms> GenerateCsv<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    pub fn new(ledger_accounts: &LedgerAccounts<Perms>) -> Self {
        Self {
            ledger_accounts: ledger_accounts.clone(),
        }
    }

    pub async fn generate_ledger_account_csv(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        ledger_account_id: LedgerAccountId,
    ) -> Result<Vec<u8>, AccountingCsvError> {
        let history_result = self
            .ledger_accounts
            .history(sub, ledger_account_id, Default::default())
            .await
            .map_err(AccountingCsvError::LedgerAccountError)?;

        let mut wtr = Writer::from_writer(vec![]);
        wtr.write_record(&[
            "Recorded At",
            "Currency",
            "Debit Amount",
            "Credit Amount",
            "Description",
            "Entry Type",
        ])
        .map_err(|e| AccountingCsvError::CsvError(e.to_string()))?;

        for entry in history_result.entities {
            let (debit_amount, credit_amount) = match entry.direction {
                DebitOrCredit::Debit => (format_amount(&entry.amount), "0".to_string()),
                DebitOrCredit::Credit => ("0".to_string(), format_amount(&entry.amount)),
            };

            wtr.write_record(&[
                entry.created_at.to_rfc3339(),
                format_currency(&entry.amount),
                debit_amount,
                credit_amount,
                entry.description.unwrap_or_default(),
                entry.entry_type,
            ])
            .map_err(|e| AccountingCsvError::CsvError(e.to_string()))?;
        }

        let csv_data = wtr
            .into_inner()
            .map_err(|e| AccountingCsvError::CsvError(e.to_string()))?;

        Ok(csv_data)
    }
}

fn format_amount(amount: &crate::journal::JournalEntryAmount) -> String {
    match amount {
        crate::journal::JournalEntryAmount::Usd(cents) => cents.to_string(),
        crate::journal::JournalEntryAmount::Btc(sats) => sats.to_string(),
    }
}

fn format_currency(amount: &crate::journal::JournalEntryAmount) -> String {
    match amount {
        crate::journal::JournalEntryAmount::Usd(_) => String::from("USD"),
        crate::journal::JournalEntryAmount::Btc(_) => String::from("BTC"),
    }
}
