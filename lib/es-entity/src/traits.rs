use serde::{de::DeserializeOwned, Serialize};

use super::{error::EsEntityError, events::EntityEvents};

pub trait EsEvent: DeserializeOwned + Serialize {
    type EntityId: Clone + PartialEq + sqlx::Type<sqlx::Postgres>;
}

pub trait IntoEvents<E: EsEvent> {
    fn into_events(self) -> EntityEvents<E>;
}

pub trait TryFromEvents<E: EsEvent> {
    fn try_from_events(events: EntityEvents<E>) -> Result<Self, EsEntityError>
    where
        Self: Sized;
}

pub trait EsEntity<E: EsEvent>: TryFromEvents<E> {
    fn events(&mut self) -> &mut EntityEvents<E>;
}
