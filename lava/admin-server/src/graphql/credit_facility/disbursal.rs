use async_graphql::*;

use crate::{
    graphql::{approval_process::*, loader::LavaDataLoader},
    primitives::*,
};
pub use lava_app::credit_facility::Disbursal as DomainDisbursal;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct CreditFacilityDisbursal {
    id: ID,
    index: DisbursalIdx,
    amount: UsdCents,
    created_at: Timestamp,

    #[graphql(skip)]
    pub(crate) entity: Arc<DomainDisbursal>,
}

impl From<DomainDisbursal> for CreditFacilityDisbursal {
    fn from(disbursement: DomainDisbursal) -> Self {
        Self {
            id: disbursement.id.to_global_id(),
            index: disbursement.idx,
            amount: disbursement.amount,
            created_at: disbursement.created_at().into(),
            entity: Arc::new(disbursement),
        }
    }
}

#[ComplexObject]
impl CreditFacilityDisbursal {
    async fn status(&self, ctx: &Context<'_>) -> async_graphql::Result<DisbursalStatus> {
        let (app, _) = crate::app_and_sub_from_ctx!(ctx);
        Ok(app
            .credit_facilities()
            .ensure_up_to_date_disbursement_status(&self.entity)
            .await?
            .map(|d| d.status())
            .unwrap_or_else(|| self.entity.status()))
    }

    async fn approval_process(&self, ctx: &Context<'_>) -> async_graphql::Result<ApprovalProcess> {
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        let process = loader
            .load_one(self.entity.approval_process_id)
            .await?
            .expect("process not found");
        Ok(process)
    }
}

#[derive(InputObject)]
pub struct CreditFacilityDisbursalInitiateInput {
    pub credit_facility_id: UUID,
    pub amount: UsdCents,
}
crate::mutation_payload! { CreditFacilityDisbursalInitiatePayload, disbursement: CreditFacilityDisbursal }

#[derive(InputObject)]
pub struct CreditFacilityDisbursalConfirmInput {
    pub credit_facility_id: UUID,
    pub disbursal_idx: DisbursalIdx,
}
crate::mutation_payload! { CreditFacilityDisbursalConfirmPayload, disbursement: CreditFacilityDisbursal }
