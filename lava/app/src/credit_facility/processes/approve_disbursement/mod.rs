mod job;

use es_entity::IntoMutableEntity;
use governance::{ApprovalProcess, ApprovalProcessStatus, ApprovalProcessType};

use crate::{
    audit::{Audit, AuditSvc},
    credit_facility::{error::CreditFacilityError, Disbursement, DisbursementRepo},
    governance::Governance,
    primitives::DisbursementId,
};
use rbac_types::{AppObject, CreditFacilityAction};

pub use job::*;

pub const APPROVE_DISBURSEMENT_PROCESS: ApprovalProcessType =
    ApprovalProcessType::new("disburseal");

#[derive(Clone)]
pub struct ApproveDisbursement {
    repo: DisbursementRepo,
    audit: Audit,
    governance: Governance,
}

impl ApproveDisbursement {
    pub fn new(repo: &DisbursementRepo, audit: &Audit, governance: &Governance) -> Self {
        Self {
            repo: repo.clone(),
            audit: audit.clone(),
            governance: governance.clone(),
        }
    }

    pub async fn execute_from_svc(
        &self,
        disbursement: impl IntoMutableEntity<Entity = Disbursement>,
    ) -> Result<Disbursement, CreditFacilityError> {
        let disbursement = disbursement.to_mutable();
        if disbursement.is_approval_process_concluded() {
            return Ok(disbursement);
        }

        let process: ApprovalProcess = self
            .governance
            .find_all_approval_processes(&[disbursement.approval_process_id])
            .await?
            .remove(&disbursement.approval_process_id)
            .expect("approval process not found");

        match process.status() {
            ApprovalProcessStatus::Approved => self.execute(disbursement, true).await,
            ApprovalProcessStatus::Denied => self.execute(disbursement, false).await,
            _ => Ok(disbursement),
        }
    }

    pub async fn execute_from_job(
        &self,
        id: impl Into<DisbursementId>,
        approved: bool,
    ) -> Result<Disbursement, CreditFacilityError> {
        let disbursement = self.repo.find_by_id(id.into()).await?;
        self.execute(disbursement, approved).await
    }

    async fn execute(
        &self,
        mut disbursement: Disbursement,
        approved: bool,
    ) -> Result<Disbursement, CreditFacilityError> {
        if disbursement.is_approval_process_concluded() {
            return Ok(disbursement);
        }
        let mut db = self.repo.pool().begin().await?;
        let audit_info = self
            .audit
            .record_system_entry_in_tx(
                &mut db,
                AppObject::CreditFacility,
                CreditFacilityAction::ConcludeDisbursementApprovalProcess,
            )
            .await?;
        disbursement.approval_process_concluded(approved, audit_info)?;
        if self.repo.update_in_tx(&mut db, &mut disbursement).await? {
            db.commit().await?;
        }
        Ok(disbursement)
    }
}
