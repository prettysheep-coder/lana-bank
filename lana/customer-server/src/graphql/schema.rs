use async_graphql::{Context, Object};

use super::authenticated_subject::*;

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, _ctx: &Context<'_>) -> async_graphql::Result<AuthenticatedSubject> {
        unimplemented!()
        // let (app, sub) = app_and_sub_from_ctx!(ctx);
        // let user = Arc::new(app.users().find_for_subject(sub).await?);
        // let loader = ctx.data_unchecked::<LanaDataLoader>();
        // loader.feed_one(user.id, User::from(user.clone())).await;
        // Ok(AuthenticatedSubject::from(user))
    }
}
