use async_graphql::*;

use crate::primitives::*;
use lava_app::user::User as DomainUser;

#[derive(InputObject)]
pub struct UserCreateInput {
    pub email: String,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    user_id: UUID,

    #[graphql(skip)]
    pub(super) entity: Arc<DomainUser>,
}

#[ComplexObject]
impl User {
    async fn roles(&self) -> Vec<LavaRole> {
        self.entity
            .current_roles()
            .into_iter()
            .map(LavaRole::from)
            .collect()
    }

    async fn email(&self) -> &str {
        &self.entity.email
    }
}

#[derive(SimpleObject)]
pub struct UserCreatePayload {
    user: User,
}

impl From<DomainUser> for User {
    fn from(user: DomainUser) -> Self {
        Self {
            user_id: UUID::from(user.id),
            entity: Arc::new(user),
        }
    }
}

impl From<Arc<DomainUser>> for User {
    fn from(user: Arc<DomainUser>) -> Self {
        Self {
            user_id: UUID::from(user.id),
            entity: user,
        }
    }
}

impl From<lava_app::user::User> for UserCreatePayload {
    fn from(user: lava_app::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}

#[derive(InputObject)]
pub struct UserAssignRoleInput {
    pub id: UUID,
    pub role: LavaRole,
}

#[derive(SimpleObject)]
pub struct UserAssignRolePayload {
    user: User,
}

impl From<lava_app::user::User> for UserAssignRolePayload {
    fn from(user: lava_app::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}

#[derive(InputObject)]
pub struct UserRevokeRoleInput {
    pub id: UUID,
    pub role: LavaRole,
}

#[derive(SimpleObject)]
pub struct UserRevokeRolePayload {
    user: User,
}

impl From<lava_app::user::User> for UserRevokeRolePayload {
    fn from(user: lava_app::user::User) -> Self {
        Self {
            user: User::from(user),
        }
    }
}
