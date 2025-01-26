pub mod error;
pub mod ledger;

use audit::AuditSvc;
use authz::PermissionCheck;
use cala_ledger::CalaLedger;
use rbac_types::{ProfitAndLossStatementAction, Subject};

use crate::{
    authorization::{Authorization, Object},
    primitives::{LedgerAccountSetId, ProfitAndLossStatementId},
    statement::*,
};

use error::*;
use ledger::*;

#[derive(Clone)]
pub struct ProfitAndLossStatements {
    pool: sqlx::PgPool,
    authz: Authorization,
    pl_statement_ledger: ProfitAndLossStatementLedger,
}

impl ProfitAndLossStatements {
    pub async fn init(
        pool: &sqlx::PgPool,
        authz: &Authorization,
        cala: &CalaLedger,
        journal_id: cala_ledger::JournalId,
    ) -> Result<Self, ProfitAndLossStatementError> {
        let pl_statement_ledger = ProfitAndLossStatementLedger::new(cala, journal_id);

        Ok(Self {
            pool: pool.clone(),
            pl_statement_ledger,
            authz: authz.clone(),
        })
    }

    pub async fn create_pl_statement(
        &self,
        id: impl Into<ProfitAndLossStatementId>,
        name: String,
    ) -> Result<LedgerAccountSetId, ProfitAndLossStatementError> {
        let account_set_id: LedgerAccountSetId = id.into().into();

        let mut op = es_entity::DbOp::init(&self.pool).await?;

        self.authz
            .audit()
            .record_system_entry_in_tx(
                op.tx(),
                Object::ProfitAndLossStatement,
                ProfitAndLossStatementAction::Create,
            )
            .await?;

        self.pl_statement_ledger
            .create(op, account_set_id, &name)
            .await?;

        Ok(account_set_id)
    }

    pub async fn find_by_name(
        &self,
        name: String,
    ) -> Result<Option<LedgerAccountSetId>, ProfitAndLossStatementError> {
        self.authz
            .audit()
            .record_system_entry(
                Object::ProfitAndLossStatement,
                ProfitAndLossStatementAction::Read,
            )
            .await?;

        let pl_statements = self
            .pl_statement_ledger
            .list_for_name(name.to_string(), Default::default())
            .await?
            .entities;

        match pl_statements.len() {
            0 => Ok(None),
            1 => Ok(Some(pl_statements[0].id)),
            _ => Err(ProfitAndLossStatementError::MultipleFound(name)),
        }
    }

    pub async fn add_to_pl_statement(
        &self,
        pl_statement_id: impl Into<ProfitAndLossStatementId>,
        member_id: impl Into<LedgerAccountSetId>,
    ) -> Result<(), ProfitAndLossStatementError> {
        let pl_statement_id = pl_statement_id.into();
        let member_id = member_id.into();

        let mut op = es_entity::DbOp::init(&self.pool).await?;

        self.authz
            .audit()
            .record_system_entry_in_tx(
                op.tx(),
                Object::ProfitAndLossStatement,
                ProfitAndLossStatementAction::Update,
            )
            .await?;

        self.pl_statement_ledger
            .add_member(op, pl_statement_id, member_id)
            .await?;

        Ok(())
    }

    pub async fn pl_statement(
        &self,
        sub: &Subject,
        name: String,
    ) -> Result<ProfitAndLossStatement, ProfitAndLossStatementError> {
        self.authz
            .enforce_permission(
                sub,
                Object::ProfitAndLossStatement,
                ProfitAndLossStatementAction::Read,
            )
            .await?;

        let pl_statement_id = self
            .find_by_name(name.to_string())
            .await?
            .ok_or(ProfitAndLossStatementError::NotFound(name))?;

        let pl_statement_details = self
            .pl_statement_ledger
            .get_pl_statement(pl_statement_id)
            .await?;

        Ok(ProfitAndLossStatement::from(pl_statement_details))
    }
}

#[derive(Clone)]
pub struct ProfitAndLossStatement {
    pub id: ProfitAndLossStatementId,
    pub name: String,
    pub description: Option<String>,
    pub btc_balance: BtcStatementAccountSetBalance,
    pub usd_balance: UsdStatementAccountSetBalance,
    pub accounts: Vec<StatementAccountSet>,
}

impl From<StatementAccountSetWithAccounts> for ProfitAndLossStatement {
    fn from(details: StatementAccountSetWithAccounts) -> Self {
        Self {
            id: details.id.into(),
            name: details.name,
            description: details.description,
            btc_balance: details.btc_balance,
            usd_balance: details.usd_balance,
            accounts: details.accounts,
        }
    }
}
