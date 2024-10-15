use chrono::{DateTime, Utc};

use super::traits::*;

pub struct PersistedEvent<E> {
    pub recorded_at: DateTime<Utc>,
    pub sequence: usize,
    pub event: E,
}

impl<E: Clone> Clone for PersistedEvent<E> {
    fn clone(&self) -> Self {
        PersistedEvent {
            recorded_at: self.recorded_at,
            sequence: self.sequence,
            event: self.event.clone(),
        }
    }
}

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

    pub fn events_persisted_at(&mut self, recorded_at: chrono::DateTime<chrono::Utc>) -> usize {
        let n = self.new_events.len();
        let offset = self.persisted_events.len() + 1;
        self.persisted_events
            .extend(
                self.new_events
                    .drain(..)
                    .enumerate()
                    .map(|(i, event)| PersistedEvent {
                        recorded_at,
                        sequence: i + offset,
                        event,
                    }),
            );
        n
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

    pub fn persisted(&self) -> impl DoubleEndedIterator<Item = &PersistedEvent<T>> {
        self.persisted_events.iter()
    }
}
