use serde::{Deserialize, Serialize};

use super::{User, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserByCreatedAtCursor {
    pub id: UserId,
}

impl From<&User> for UserByCreatedAtCursor {
    fn from(values: &User) -> Self {
        Self { id: values.id }
    }
}
