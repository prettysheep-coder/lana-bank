use serde::{de::DeserializeOwned, Serialize};

use super::events::EntityEvents;

// pub trait EsEntity {
//     type EntityId;
//     type Event;
// }

pub trait EsEvent: DeserializeOwned + Serialize {
    type EntityId: Clone;
}

pub trait IntoEvents<E: EsEvent> {
    fn into_events(self) -> EntityEvents<E>;
}
// pub trait FromEvents {
//     type Event;
//     type Error = EntityError;

//     fn from_events(events: Vec<Event>) -> Result<Self, Self::Error>
//     where
//         Self: Sized;
// }
