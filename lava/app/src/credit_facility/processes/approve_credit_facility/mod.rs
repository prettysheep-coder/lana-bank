mod job;

use es_entity::IntoMutableEntity;
use governance::{ApprovalProcess, ApprovalProcessStatus, ApprovalProcessType};

use crate::{
    audit::{Audit, AuditSvc},
    credit_facility::{
        activate, error::CreditFacilityError, CreditFacility, CreditFacilityRepo,
        InterestAccrualRepo,
    },
    governance::Governance,
    job::Jobs,
    ledger::Ledger,
    price::Price,
    primitives::CreditFacilityId,
};
use rbac_types::{AppObject, CreditFacilityAction};

pub use job::*;

pub const APPROVE_CREDIT_FACILITY_PROCESS: ApprovalProcessType =
    ApprovalProcessType::new("credit-facility");

#[derive(Clone)]
pub struct ApproveCreditFacility {
    repo: CreditFacilityRepo,
    interest_accrual_repo: InterestAccrualRepo,
    ledger: Ledger,
    jobs: Jobs,
    governance: Governance,
    audit: Audit,
    price: Price,
}

impl ApproveCreditFacility {
    pub(in crate::credit_facility) fn new(
        repo: &CreditFacilityRepo,
        interest_accrual_repo: &InterestAccrualRepo,
        ledger: &Ledger,
        price: &Price,
        jobs: &Jobs,
        audit: &Audit,
        governance: &Governance,
    ) -> Self {
        Self {
            repo: repo.clone(),
            interest_accrual_repo: interest_accrual_repo.clone(),
            ledger: ledger.clone(),
            price: price.clone(),
            jobs: jobs.clone(),
            audit: audit.clone(),
            governance: governance.clone(),
        }
    }

    pub async fn execute_from_svc(
        &self,
        credit_facility: impl IntoMutableEntity<Entity = CreditFacility>,
    ) -> Result<CreditFacility, CreditFacilityError> {
        let credit_facility = credit_facility.to_mutable();
        if credit_facility.is_approval_process_concluded() {
            return Ok(credit_facility);
        }

        let process: ApprovalProcess = self
            .governance
            .find_all_approval_processes(&[credit_facility.approval_process_id])
            .await?
            .remove(&credit_facility.approval_process_id)
            .expect("approval process not found");

        match process.status() {
            ApprovalProcessStatus::Approved => self.execute(credit_facility, true).await,
            ApprovalProcessStatus::Denied => self.execute(credit_facility, false).await,
            _ => Ok(credit_facility),
        }
    }

    pub async fn execute_from_job(
        &self,
        id: impl Into<CreditFacilityId>,
        approved: bool,
    ) -> Result<CreditFacility, CreditFacilityError> {
        let credit_facility = self.repo.find_by_id(id.into()).await?;
        self.execute(credit_facility, approved).await
    }

    async fn execute(
        &self,
        mut credit_facility: CreditFacility,
        approved: bool,
    ) -> Result<CreditFacility, CreditFacilityError> {
        if credit_facility.is_approval_process_concluded() {
            return Ok(credit_facility);
        }
        let mut db = self.repo.pool().begin().await?;
        let audit_info = self
            .audit
            .record_system_entry_in_tx(
                &mut db,
                AppObject::CreditFacility,
                CreditFacilityAction::ConcludeApprovalProcess,
            )
            .await?;
        credit_facility.approval_process_concluded(approved, audit_info);

        let price = self.price.usd_cents_per_btc().await?;
        activate::execute(
            &mut credit_facility,
            &mut db,
            &self.ledger,
            &self.audit,
            self.interest_accrual_repo.clone(),
            &self.jobs,
            price,
        )
        .await?;
        if self
            .repo
            .update_in_tx(&mut db, &mut credit_facility)
            .await?
        {
            db.commit().await?;
        }
        Ok(credit_facility)
    }
}
