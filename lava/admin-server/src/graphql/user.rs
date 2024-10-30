use async_graphql::*;

use crate::primitives::*;
use lava_app::user::User as DomainUser;

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

#[derive(InputObject)]
pub struct UserCreateInput {
    pub email: String,
}

mutation_payload! { UserCreatePayload, user: User }

#[derive(InputObject)]
pub struct UserAssignRoleInput {
    pub id: UUID,
    pub role: LavaRole,
}
mutation_payload! { UserAssignRolePayload, user: User }

#[derive(InputObject)]
pub struct UserRevokeRoleInput {
    pub id: UUID,
    pub role: LavaRole,
}

mutation_payload! { UserRevokeRolePayload, user: User }
