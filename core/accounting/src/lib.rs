#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

pub mod chart_of_accounts;
pub mod error;
pub mod journal;
pub mod ledger_account;
mod primitives;
pub mod statement;
pub mod trial_balance;

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::CalaLedger;
use tracing::instrument;

pub use chart_of_accounts::{Chart, ChartOfAccounts, error as chart_of_accounts_error, tree};
use error::CoreAccountingError;
pub use journal::{Journal, error as journal_error};
pub use ledger_account::{LedgerAccount, LedgerAccounts};
pub use primitives::*;
pub use trial_balance::{
    TrialBalanceAccount, TrialBalanceAccountCursor, TrialBalanceRoot, TrialBalances,
};

pub struct CoreAccounting<Perms>
where
    Perms: PermissionCheck,
{
    authz: Perms,
    chart_of_accounts: ChartOfAccounts<Perms>,
    journal: Journal<Perms>,
    ledger_accounts: LedgerAccounts<Perms>,
    trial_balances: TrialBalances<Perms>,
}

impl<Perms> Clone for CoreAccounting<Perms>
where
    Perms: PermissionCheck,
{
    fn clone(&self) -> Self {
        Self {
            authz: self.authz.clone(),
            chart_of_accounts: self.chart_of_accounts.clone(),
            journal: self.journal.clone(),
            ledger_accounts: self.ledger_accounts.clone(),
            trial_balances: self.trial_balances.clone(),
        }
    }
}

impl<Perms> CoreAccounting<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    pub fn new(
        pool: &sqlx::PgPool,
        authz: &Perms,
        cala: &CalaLedger,
        journal_id: CalaJournalId,
    ) -> Self {
        let chart_of_accounts = ChartOfAccounts::new(pool, authz, cala, journal_id);
        let journal = Journal::new(authz, cala, journal_id);
        let ledger_accounts = LedgerAccounts::new(authz, cala, journal_id);
        let trial_balances = TrialBalances::new(pool, authz, cala, journal_id);
        Self {
            authz: authz.clone(),
            chart_of_accounts,
            journal,
            ledger_accounts,
            trial_balances,
        }
    }

    pub fn chart_of_accounts(&self) -> &ChartOfAccounts<Perms> {
        &self.chart_of_accounts
    }

    pub fn journal(&self) -> &Journal<Perms> {
        &self.journal
    }

    pub fn ledger_accounts(&self) -> &LedgerAccounts<Perms> {
        &self.ledger_accounts
    }

    pub fn trial_balances(&self) -> &TrialBalances<Perms> {
        &self.trial_balances
    }

    #[instrument(name = "core_accounting.find_ledger_account_by_code", skip(self))]
    pub async fn find_ledger_account_by_code(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        chart_ref: &str,
        code: String,
    ) -> Result<Option<LedgerAccount>, CoreAccountingError> {
        let chart_ref = chart_ref.to_string();
        let chart = self
            .chart_of_accounts
            .find_by_reference(sub, &chart_ref)
            .await?
            .ok_or_else(move || {
                CoreAccountingError::ChartOfAccountsNotFoundByReference(chart_ref)
            })?;
        Ok(self
            .ledger_accounts
            .find_by_code(sub, chart.id, code.parse()?)
            .await?)
    }
}
