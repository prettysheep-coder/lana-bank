use serde::{de::DeserializeOwned, Serialize};

use super::{error::EsEntityError, events::EntityEvents};

pub trait EsEvent: DeserializeOwned + Serialize {
    type EntityId: Clone + PartialEq + sqlx::Type<sqlx::Postgres> + std::hash::Hash;
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

pub trait AsMutableEntity {
    type Entity: EsEntity;
    fn mutable(&self) -> Self::Entity;
}

impl<T, E> AsMutableEntity for T
where
    T: EsEntity<Event = E>,
    E: EsEvent + Clone,
{
    type Entity = T;

    fn mutable(&self) -> Self::Entity {
        <T as TryFromEvents<E>>::try_from_events(self.events().clone())
            .expect("Could not convert events to entity")
    }
}
