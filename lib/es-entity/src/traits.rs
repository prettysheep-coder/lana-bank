use serde::{de::DeserializeOwned, Serialize};

use super::{error::EsEntityError, events::EntityEvents};

pub trait EsEvent: DeserializeOwned + Serialize + Send + Sync {
    type EntityId: Clone + PartialEq + sqlx::Type<sqlx::Postgres> + std::hash::Hash + Send + Sync;
}

pub trait IntoEvents<E: EsEvent> {
    fn into_events(self) -> EntityEvents<E>;
}

pub trait TryFromEvents<E: EsEvent> {
    fn try_from_events(events: EntityEvents<E>) -> Result<Self, EsEntityError>
    where
        Self: Sized;
}

pub trait EsEntity: TryFromEvents<Self::Event> {
    type Event: EsEvent;

    fn events_mut(&mut self) -> &mut EntityEvents<Self::Event>;
    fn events(&self) -> &EntityEvents<Self::Event>;
}

pub trait RetryableInto<T>: Into<T> + Copy + std::fmt::Debug {}
impl<T, O> RetryableInto<O> for T where T: Into<O> + Copy + std::fmt::Debug {}
