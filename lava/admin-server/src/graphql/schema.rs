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

    async fn user(&self, ctx: &Context<'_>, id: UUID) -> async_graphql::Result<Option<User>> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        if let Some(user) = app.users().find_by_id(sub, UserId::from(id)).await? {
            let user = User::from(user);
            let loader = ctx.data_unchecked::<LavaDataLoader>();
            loader.feed_one(user.entity.id, user.clone()).await;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        let users: Vec<_> = app
            .users()
            .list_users(sub)
            .await?
            .into_iter()
            .map(User::from)
            .collect();
        loader
            .feed_many(users.iter().map(|u| (u.entity.id, u.clone())))
            .await;
        Ok(users)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello(&self) -> String {
        "world".to_string()
    }
}
