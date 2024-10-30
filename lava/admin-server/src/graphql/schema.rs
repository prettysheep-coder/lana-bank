use async_graphql::{types::connection::*, Context, Object};

use lava_app::app::LavaApp;

use crate::primitives::*;

use super::{authenticated_subject::*, user::*};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<AuthenticatedSubject> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let user = app.users().find_for_subject(sub).await?;
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
