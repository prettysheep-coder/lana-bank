use async_graphql::{Context, Object};

use crate::primitives::*;

use super::{authenticated_subject::*, credit_facility::*};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<AuthenticatedSubject> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let customer = app.customers().find_for_subject(sub).await?;
        Ok(AuthenticatedSubject::from(customer))
    }

    async fn credit_facility(
        &self,
        ctx: &Context<'_>,
        id: UUID,
    ) -> async_graphql::Result<Option<CreditFacility>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);

        Ok(app
            .credit_facilities()
            .for_subject(sub)?
            .find_by_id(id)
            .await?
            .map(CreditFacility::from))
    }
}
