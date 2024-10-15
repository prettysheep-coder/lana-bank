use chrono::{DateTime, Utc};

use super::traits::*;

#[derive(Clone)]
pub struct PersistedEvent<E> {
    recorded_at: DateTime<Utc>,
    event: E,
}

#[derive(Clone)]
pub struct EntityEvents<T: EsEvent> {
    pub entity_id: <T as EsEvent>::EntityId,
    persisted_events: Vec<PersistedEvent<T>>,
    new_events: Vec<T>,
}

impl<T> EntityEvents<T>
where
    T: EsEvent,
{
    pub fn init(id: <T as EsEvent>::EntityId, initial_events: impl IntoIterator<Item = T>) -> Self {
        Self {
            entity_id: id,
            persisted_events: Vec::new(),
            new_events: initial_events.into_iter().collect(),
        }
    }
}
