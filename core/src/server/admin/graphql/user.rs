use async_graphql::*;

use crate::{
    app::LavaApp,
    primitives::UserId,
    server::{admin::AdminAuthContext, shared_graphql::primitives::UUID},
};

#[derive(InputObject)]
pub struct UserCreateInput {
    pub email: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    user_id: UUID,
    email: String,
}

#[ComplexObject]
impl User {
    async fn roles(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Role>> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;

        let roles = app
            .users()
            .roles_for_user(sub, UserId::from(&self.user_id))
            .await?;
        Ok(roles.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(SimpleObject)]
pub struct UserCreatePayload {
    user: User,
}

impl From<crate::user::User> for User {
    fn from(user: crate::user::User) -> Self {
        Self {
            user_id: UUID::from(user.id),
            email: user.email,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::primitives::Role")]
pub enum Role {
    Superuser,
    Admin,
    BankManager,
}

impl From<crate::user::User> for UserCreatePayload {
    fn from(user: crate::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}

#[derive(InputObject)]
pub struct UserAssignRoleInput {
    pub id: UUID,
    pub role: Role,
}

#[derive(SimpleObject)]
pub struct UserAssignRolePayload {
    user: User,
}

impl From<crate::user::User> for UserAssignRolePayload {
    fn from(user: crate::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}

#[derive(InputObject)]
pub struct UserRevokeRoleInput {
    pub id: UUID,
    pub role: Role,
}

#[derive(SimpleObject)]
pub struct UserRevokeRolePayload {
    user: User,
}

impl From<crate::user::User> for UserRevokeRolePayload {
    fn from(user: crate::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}
