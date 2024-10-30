use async_graphql::*;

use crate::primitives::*;

use super::terms::*;

use lava_app::terms_template::TermsTemplate as DomainTermsTemplate;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct TermsTemplate {
    id: ID,
    terms_id: UUID,
    values: TermValues,
    created_at: Timestamp,

    #[graphql(skip)]
    pub(super) entity: Arc<DomainTermsTemplate>,
}

impl From<DomainTermsTemplate> for TermsTemplate {
    fn from(terms: DomainTermsTemplate) -> Self {
        Self {
            id: terms.id.to_global_id(),
            created_at: terms.created_at().into(),
            terms_id: terms.id.into(),
            values: terms.values.into(),
            entity: Arc::new(terms),
        }
    }
}

#[ComplexObject]
impl TermsTemplate {
    async fn name(&self) -> &str {
        &self.entity.name
    }
}

#[derive(InputObject)]
pub(super) struct TermsTemplateCreateInput {
    pub name: String,
    pub annual_rate: AnnualRatePct,
    pub accrual_interval: InterestInterval,
    pub incurrence_interval: InterestInterval,
    pub duration: DurationInput,
    pub liquidation_cvl: CVLPct,
    pub margin_call_cvl: CVLPct,
    pub initial_cvl: CVLPct,
}
crate::mutation_payload! { TermsTemplateCreatePayload, terms_template: TermsTemplate }

#[derive(InputObject)]
pub(super) struct TermsTemplateUpdateInput {
    pub id: UUID,
    pub annual_rate: AnnualRatePct,
    pub accrual_interval: InterestInterval,
    pub incurrence_interval: InterestInterval,
    pub liquidation_cvl: CVLPct,
    pub duration: DurationInput,
    pub margin_call_cvl: CVLPct,
    pub initial_cvl: CVLPct,
}
crate::mutation_payload! { TermsTemplateUpdatePayload, terms_template: TermsTemplate }
