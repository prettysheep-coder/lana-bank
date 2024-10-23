mod committee;
pub mod error;
mod process_assignment;

use tracing::instrument;

use crate::{
    authorization::{Authorization, CommitteeAction, Object},
    data_export::Export,
    primitives::{ApprovalProcessType, CommitteeId, Subject, UserId},
};

pub use committee::*;
use error::*;

#[derive(Clone)]
pub struct Governance {
    pool: sqlx::PgPool,
    committee_repo: CommitteeRepo,
    authz: Authorization,
}

impl Governance {
    pub fn new(pool: &sqlx::PgPool, authz: &Authorization, export: &Export) -> Governance {
        let committee_repo = CommitteeRepo::new(pool, export);
        Governance {
            pool: pool.clone(),
            committee_repo,
            authz: authz.clone(),
        }
    }

    #[instrument(name = "lava.governance.create_committee", skip(self), err)]
    pub async fn create_committee(
        &self,
        sub: &Subject,
        approval_process_type: ApprovalProcessType,
    ) -> Result<Committee, GovernanceError> {
        let audit_info = self
            .authz
            .evaluate_permission(sub, Object::Committee, CommitteeAction::Create, true)
            .await?
            .expect("audit info missing");

        let new_committee = NewCommittee::builder()
            .id(CommitteeId::new())
            .audit_info(audit_info)
            .build()
            .expect("Could not build new committee");

        let mut db = self.pool.begin().await?;
        let committee = self
            .committee_repo
            .create_in_tx(&mut db, new_committee)
            .await?;
        db.commit().await?;
        Ok(committee)
    }

    #[instrument(name = "lava.governance.add_user_to_committee", skip(self), err)]
    pub async fn add_user_to_committee(
        &self,
        sub: &Subject,
        committee_id: impl Into<CommitteeId> + std::fmt::Debug,
        user_id: impl Into<UserId> + std::fmt::Debug,
    ) -> Result<Committee, GovernanceError> {
        let audit_info = self
            .authz
            .evaluate_permission(sub, Object::Committee, CommitteeAction::AddUser, true)
            .await?
            .expect("audit info missing");

        let mut committee = self.committee_repo.find_by_id(committee_id.into()).await?;

        committee.add_user(user_id.into(), audit_info);

        self.committee_repo.update(&mut committee).await?;

        Ok(committee)
    }

    #[instrument(name = "lava.governance.remove_user_from_committee", skip(self), err)]
    pub async fn remove_user_from_committee(
        &self,
        sub: &Subject,
        committee_id: impl Into<CommitteeId> + std::fmt::Debug,
        user_id: impl Into<UserId> + std::fmt::Debug,
    ) -> Result<Committee, GovernanceError> {
        let audit_info = self
            .authz
            .evaluate_permission(sub, Object::Committee, CommitteeAction::RemoveUser, true)
            .await?
            .expect("audit info missing");

        let mut committee = self.committee_repo.find_by_id(committee_id.into()).await?;

        committee.remove_user(user_id.into(), audit_info);

        self.committee_repo.update(&mut committee).await?;

        Ok(committee)
    }

    //pub fn create_committee() => ApprovalProcessType
    //pub fn add_user_to_committee()
    //pub fn remove_user_from_committee()
    //
    //in GQL User.committeed {}
    //
    //pub fn create_approval_process(type: ProcessType) {
    //  self.find_committee_for_process_type()
    //}
    //NewApprovalProcessBuilder::new().process_type(ApprovalProcessType::CreditFacilityApproval).committee(CommitteeId::new()).build()
    //NewApprovalProcessBuilder::new().process_type(ApprovalProcessType::CreditFacilityDisbursement).build()
    //pub fn add_approval()
}
