pub mod constants;
mod seed;

pub mod error;

use crate::{accounting::ChartOfAccounts, primitives::CalaJournalId};

use cala_ledger::CalaLedger;
use error::*;

#[derive(Clone)]
pub struct JournalInit {
    pub journal_id: CalaJournalId,
}

impl JournalInit {
    pub async fn journal(cala: &CalaLedger) -> Result<Self, AccountingInitError> {
        seed::journal::init(cala).await
    }
}

#[derive(Clone)]
pub struct ChartsInit;

impl ChartsInit {
    pub async fn charts_of_accounts(
        chart_of_accounts: &ChartOfAccounts,
    ) -> Result<(), AccountingInitError> {
        seed::charts_of_accounts::init(chart_of_accounts).await
    }
}
