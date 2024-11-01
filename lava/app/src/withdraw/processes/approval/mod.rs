mod job;

use es_entity::IntoMutableEntity;
use governance::{ApprovalProcess, ApprovalProcessStatus, ApprovalProcessType};

use crate::{
    audit::{Audit, AuditSvc},
    governance::Governance,
    primitives::WithdrawId,
    withdraw::{error::WithdrawError, repo::WithdrawRepo, Withdraw},
};
use rbac_types::{AppObject, WithdrawAction};

pub use job::*;

pub const APPROVE_WITHDRAW_PROCESS: ApprovalProcessType = ApprovalProcessType::new("withdraw");
pub async fn execute() {}

#[derive(Clone)]
pub struct ApproveWithdraw {
    repo: WithdrawRepo,
    audit: Audit,
    governance: Governance,
}

impl ApproveWithdraw {
    pub fn new(repo: &WithdrawRepo, audit: &Audit, governance: &Governance) -> Self {
        Self {
            repo: repo.clone(),
            audit: audit.clone(),
            governance: governance.clone(),
        }
    }

    pub async fn execute_from_svc(
        &self,
        withdraw: impl IntoMutableEntity<Entity = Withdraw>,
    ) -> Result<Withdraw, WithdrawError> {
        let withdraw = withdraw.to_mutable();
        if withdraw.is_approved_or_denied().is_some() {
            return Ok(withdraw);
        }

        let process: ApprovalProcess = self
            .governance
            .find_all_approval_processes(&[withdraw.approval_process_id])
            .await?
            .remove(&withdraw.approval_process_id)
            .expect("approval process not found");

        match process.status() {
            ApprovalProcessStatus::Approved => self.execute(withdraw, true).await,
            ApprovalProcessStatus::Denied => self.execute(withdraw, false).await,
            _ => Ok(withdraw),
        }
    }

    pub async fn execute_from_job(
        &self,
        id: impl Into<WithdrawId>,
        approved: bool,
    ) -> Result<Withdraw, WithdrawError> {
        let withdraw = self.repo.find_by_id(id.into()).await?;
        self.execute(withdraw, approved).await
    }

    async fn execute(
        &self,
        mut withdraw: Withdraw,
        approved: bool,
    ) -> Result<Withdraw, WithdrawError> {
        if withdraw.is_approved_or_denied().is_some() {
            return Ok(withdraw);
        }
        let mut db = self.repo.pool().begin().await?;
        let audit_info = self
            .audit
            .record_system_entry_in_tx(
                &mut db,
                AppObject::Withdraw,
                WithdrawAction::ConcludeApprovalProcess,
            )
            .await?;
        withdraw.approval_process_concluded(approved, audit_info)?;
        if self.repo.update_in_tx(&mut db, &mut withdraw).await? {
            db.commit().await?;
        }
        Ok(withdraw)
    }
}
