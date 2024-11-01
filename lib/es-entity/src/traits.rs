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

pub trait EsEntity<E: EsEvent>: TryFromEvents<E> {
    fn events_mut(&mut self) -> &mut EntityEvents<E>;
    fn events(&self) -> &EntityEvents<E>;
}

pub trait IntoMutableEntity<E: EsEvent> {
    type Entity: EsEntity<E>;
    fn to_mutable(self) -> Self::Entity;
}

impl<'a, T, E> IntoMutableEntity<E> for &'a T
where
    E: EsEvent + Clone,
    T: EsEntity<E>,
{
    type Entity = T;

    fn to_mutable(self) -> Self::Entity {
        <T as TryFromEvents<E>>::try_from_events(self.events().clone())
            .expect("Could not convert events to entity")
    }
}
