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
    async fn user_create(
        &self,
        ctx: &Context<'_>,
        input: UserCreateInput,
    ) -> async_graphql::Result<UserCreatePayload> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let user = app.users().create_user(sub, input.email).await?;
        Ok(UserCreatePayload::from(user))
    }

    async fn user_assign_role(
        &self,
        ctx: &Context<'_>,
        input: UserAssignRoleInput,
    ) -> async_graphql::Result<UserAssignRolePayload> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let UserAssignRoleInput { id, role } = input;
        let user = app.users().assign_role_to_user(sub, id, role).await?;
        Ok(UserAssignRolePayload::from(user))
    }

    async fn user_revoke_role(
        &self,
        ctx: &Context<'_>,
        input: UserRevokeRoleInput,
    ) -> async_graphql::Result<UserRevokeRolePayload> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let UserRevokeRoleInput { id, role } = input;
        let user = app.users().revoke_role_from_user(sub, id, role).await?;
        Ok(UserRevokeRolePayload::from(user))
    }
}
