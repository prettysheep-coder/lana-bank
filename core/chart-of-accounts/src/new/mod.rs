mod csv;
mod primitives;

use audit::AuditSvc;
use authz::PermissionCheck;

use cala_ledger::{account_set::NewAccountSet, CalaLedger};
use tracing::instrument;

use super::error::*;
use primitives::*;

pub struct CoreChartOfAccounts<Perms>
where
    Perms: PermissionCheck,
{
    // repo: ChartRepo,
    cala: CalaLedger,
    authz: Perms,
    journal_id: LedgerJournalId,
}

impl<Perms> Clone for CoreChartOfAccounts<Perms>
where
    Perms: PermissionCheck,
{
    fn clone(&self) -> Self {
        Self {
            // repo: self.repo.clone(),
            cala: self.cala.clone(),
            authz: self.authz.clone(),
            journal_id: self.journal_id,
        }
    }
}

impl<Perms> CoreChartOfAccounts<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreChartOfAccountsAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreChartOfAccountsObject>,
{
    pub async fn init(
        pool: &sqlx::PgPool,
        authz: &Perms,
        cala: &CalaLedger,
        journal_id: LedgerJournalId,
    ) -> Result<Self, CoreChartOfAccountsError> {
        // let chart_of_account = ChartRepo::new(pool);
        let res = Self {
            // repo: chart_of_account,
            cala: cala.clone(),
            authz: authz.clone(),
            journal_id,
        };
        Ok(res)
    }

    // #[instrument(name = "chart_of_accounts.create_chart", skip(self))]
    // pub async fn create_chart(
    //     &self,
    //     id: impl Into<ChartId> + std::fmt::Debug,
    //     name: String,
    //     reference: String,
    // ) -> Result<Chart, CoreChartOfAccountsError> {
    //     let id = id.into();

    //     let mut op = self.repo.begin_op().await?;
    //     let audit_info = self
    //         .authz
    //         .audit()
    //         .record_system_entry_in_tx(
    //             op.tx(),
    //             CoreChartOfAccountsObject::chart(id),
    //             CoreChartOfAccountsAction::CHART_CREATE,
    //         )
    //         .await?;

    //     let new_chart_of_account = NewChart::builder()
    //         .id(id)
    //         .name(name)
    //         .reference(reference)
    //         .audit_info(audit_info)
    //         .build()
    //         .expect("Could not build new chart of accounts");

    //     let chart = self
    //         .repo
    //         .create_in_op(&mut op, new_chart_of_account)
    //         .await?;
    //     op.commit().await?;

    //     Ok(chart)
    // }
}
