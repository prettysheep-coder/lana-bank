use serde::{Deserialize, Serialize};

use super::ids::UserId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserModuleEvent {
    UserCreated { id: UserId },
    UserRemoved { id: UserId },
}
