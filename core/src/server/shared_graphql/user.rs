use async_graphql::*;

use crate::server::shared_graphql::primitives::UUID;

#[derive(SimpleObject)]
pub struct User {
    user_id: UUID,
    email: String,
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
    SuperUser,
    BankManager,
}
