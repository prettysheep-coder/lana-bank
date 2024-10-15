use chrono::{DateTime, Utc};

use super::traits::*;

#[derive(Clone)]
pub struct PersistedEvent<E> {
    recorded_at: DateTime<Utc>,
    sequence: u64,
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

    pub fn id(&self) -> &<T as EsEvent>::EntityId {
        &self.entity_id
    }

    pub fn serialize_new_events(&self) -> Vec<serde_json::Value> {
        self.new_events
            .iter()
            .map(|event| serde_json::to_value(event).expect("Failed to serialize event"))
            .collect()
    }

    pub fn any_new(&self) -> bool {
        !self.new_events.is_empty()
    }

    pub fn n_persisted(&self) -> usize {
        self.persisted_events.len()
    }
}
