use async_graphql::*;

use crate::{
    graphql::{loader::LavaDataLoader, user::*},
    primitives::*,
};
pub use lava_app::credit_facility::Disbursement as DomainDisbursement;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct CreditFacilityDisbursement {
    id: ID,
    index: DisbursementIdx,
    amount: UsdCents,
    status: DisbursementStatus,
    created_at: Timestamp,

    #[graphql(skip)]
    pub(crate) entity: Arc<DomainDisbursement>,
}

impl From<DomainDisbursement> for CreditFacilityDisbursement {
    fn from(disbursement: DomainDisbursement) -> Self {
        Self {
            id: disbursement.id.to_global_id(),
            index: disbursement.idx,
            amount: disbursement.amount,
            status: disbursement.status(),
            created_at: disbursement.created_at().into(),
            entity: Arc::new(disbursement),
        }
    }
}

#[ComplexObject]
impl CreditFacilityDisbursement {
    async fn approvals(&self) -> Vec<DisbursementApproval> {
        self.entity
            .approvals()
            .into_iter()
            .map(DisbursementApproval::from)
            .collect()
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DisbursementApproval {
    approved_at: Timestamp,
    #[graphql(skip)]
    user_id: lava_app::primitives::UserId,
}

impl From<lava_app::credit_facility::DisbursementApproval> for DisbursementApproval {
    fn from(disbursement_approval: lava_app::credit_facility::DisbursementApproval) -> Self {
        Self {
            user_id: disbursement_approval.user_id,
            approved_at: disbursement_approval.approved_at.into(),
        }
    }
}

#[ComplexObject]
impl DisbursementApproval {
    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        let user = loader
            .load_one(self.user_id)
            .await?
            .expect("committee not found");
        Ok(user)
    }
}

#[derive(InputObject)]
pub struct CreditFacilityDisbursementInitiateInput {
    pub credit_facility_id: UUID,
    pub amount: UsdCents,
}
crate::mutation_payload! { CreditFacilityDisbursementInitiatePayload, disbursement: CreditFacilityDisbursement }
