use serde::{Deserialize, Serialize};

use es_entity::{EntityEvents, EsEvent, IntoEvents};

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, Deserialize, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct UserId(uuid::Uuid);

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(EsEvent, Deserialize, Serialize)]
#[es_event(id = "UserId")]
pub enum UserEvent {
    Initialized { id: UserId, email: String },
}

#[derive(Debug)]
pub struct NewUser {
    pub id: UserId,
    pub email: String,
}

impl IntoEvents<UserEvent> for NewUser {
    fn into_events(self) -> EntityEvents<UserEvent> {
        EntityEvents::init(
            self.id,
            vec![UserEvent::Initialized {
                id: self.id,
                email: self.email,
            }],
        )
    }
}

pub struct User {
    pub id: UserId,
    pub email: String,
}
