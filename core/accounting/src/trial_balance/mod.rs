pub mod error;
pub mod ledger;

use chrono::{DateTime, Utc};

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::CalaLedger;

use crate::{
    chart_of_accounts::Chart,
    primitives::{CoreAccountingAction, CoreAccountingObject},
};

use error::*;
use ledger::*;
pub use ledger::{TrialBalanceAccount, TrialBalanceAccountCursor, TrialBalanceRoot};

#[derive(Clone)]
pub struct TrialBalances<Perms>
where
    Perms: PermissionCheck,
{
    pool: sqlx::PgPool,
    authz: Perms,
    trial_balance_ledger: TrialBalanceLedger,
}

impl<Perms> TrialBalances<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreAccountingAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreAccountingObject>,
{
    pub fn new(
        pool: &sqlx::PgPool,
        authz: &Perms,
        cala: &CalaLedger,
        journal_id: cala_ledger::JournalId,
    ) -> Self {
        let trial_balance_ledger = TrialBalanceLedger::new(cala, journal_id);

        Self {
            pool: pool.clone(),
            trial_balance_ledger,
            authz: authz.clone(),
        }
    }

    pub async fn create_trial_balance_statement(
        &self,
        reference: String,
    ) -> Result<(), TrialBalanceError> {
        let mut op = es_entity::DbOp::init(&self.pool).await?;

        self.authz
            .audit()
            .record_system_entry_in_tx(
                op.tx(),
                CoreAccountingObject::all_trial_balance(),
                CoreAccountingAction::TRIAL_BALANCE_CREATE,
            )
            .await?;

        match self.trial_balance_ledger.create(op, &reference).await {
            Ok(_) => Ok(()),
            Err(e) if e.account_set_exists() => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn add_chart_to_trial_balance(
        &self,
        name: String,
        chart: Chart,
    ) -> Result<(), TrialBalanceError> {
        let trial_balance_id = self
            .trial_balance_ledger
            .get_id_from_reference(name)
            .await?;

        let mut op = es_entity::DbOp::init(&self.pool).await?;

        self.authz
            .audit()
            .record_system_entry_in_tx(
                op.tx(),
                CoreAccountingObject::trial_balance(trial_balance_id.into()),
                CoreAccountingAction::TRIAL_BALANCE_UPDATE,
            )
            .await?;

        self.trial_balance_ledger
            .add_members(
                op,
                trial_balance_id,
                chart.all_trial_balance_accounts().map(|(_, id)| *id),
            )
            .await?;

        Ok(())
    }

    pub async fn trial_balance(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        name: String,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<TrialBalanceRoot, TrialBalanceError> {
        let trial_balance_id = self
            .trial_balance_ledger
            .get_id_from_reference(name.clone())
            .await?;

        self.authz
            .enforce_permission(
                sub,
                CoreAccountingObject::trial_balance(trial_balance_id.into()),
                CoreAccountingAction::TRIAL_BALANCE_READ,
            )
            .await?;

        Ok(self
            .trial_balance_ledger
            .get_trial_balance(name, from, Some(until))
            .await?)
    }

    pub async fn trial_balance_accounts(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        name: String,
        from: DateTime<Utc>,
        until: Option<DateTime<Utc>>,
        args: es_entity::PaginatedQueryArgs<TrialBalanceAccountCursor>,
    ) -> Result<
        es_entity::PaginatedQueryRet<TrialBalanceAccount, TrialBalanceAccountCursor>,
        TrialBalanceError,
    > {
        let trial_balance_id = self
            .trial_balance_ledger
            .get_id_from_reference(name.clone())
            .await?;

        self.authz
            .enforce_permission(
                sub,
                CoreAccountingObject::trial_balance(trial_balance_id.into()),
                CoreAccountingAction::TRIAL_BALANCE_READ,
            )
            .await?;

        Ok(self
            .trial_balance_ledger
            .accounts(name, from, until, args)
            .await?)
    }
}
