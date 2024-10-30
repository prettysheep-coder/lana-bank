use async_graphql::{Context, Object};

use lava_app::app::LavaApp;

use crate::primitives::*;

use super::{authenticated_subject::*, loader::*, user::*};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<AuthenticatedSubject> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let user = Arc::new(app.users().find_for_subject(sub).await?);
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        loader.feed_one(user.id, User::from(user.clone())).await;
        Ok(AuthenticatedSubject::from(user))
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello(&self) -> String {
        "world".to_string()
    }
}
