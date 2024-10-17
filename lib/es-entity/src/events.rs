use chrono::{DateTime, Utc};

use super::{error::EsEntityError, traits::*};

pub struct GenericEvent<Id> {
    pub id: Id,
    pub sequence: i32,
    pub event: serde_json::Value,
    pub recorded_at: DateTime<Utc>,
}

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

    pub fn load_first<E: EsEntity<T>>(
        events: impl IntoIterator<Item = GenericEvent<<T as EsEvent>::EntityId>>,
    ) -> Result<E, EsEntityError> {
        let mut current_id = None;
        let mut current = None;
        for e in events {
            if current_id.is_none() {
                current_id = Some(e.id.clone());
                current = Some(Self {
                    entity_id: e.id.clone(),
                    persisted_events: Vec::new(),
                    new_events: Vec::new(),
                });
            }
            if current_id != Some(e.id) {
                break;
            }
            let cur = current.as_mut().expect("Could not get current");
            cur.persisted_events.push(PersistedEvent {
                recorded_at: e.recorded_at,
                sequence: e.sequence as usize,
                event: serde_json::from_value(e.event).expect("Could not deserialize event"),
            });
        }
        if let Some(current) = current {
            E::try_from_events(current)
        } else {
            Err(EsEntityError::NotFound)
        }
    }

    pub fn load_n<E: EsEntity<T>>(
        events: impl IntoIterator<Item = GenericEvent<<T as EsEvent>::EntityId>>,
        n: usize,
    ) -> Result<(Vec<E>, bool), EsEntityError> {
        let mut ret: Vec<E> = Vec::new();
        let mut current_id = None;
        let mut current = None;
        for e in events {
            if current_id.as_ref() != Some(&e.id) {
                if let Some(current) = current.take() {
                    ret.push(E::try_from_events(current)?);
                    if ret.len() == n {
                        return Ok((ret, true));
                    }
                }

                current_id = Some(e.id.clone());
                current = Some(Self {
                    entity_id: e.id,
                    persisted_events: Vec::new(),
                    new_events: Vec::new(),
                });
            }
            let cur = current.as_mut().expect("Could not get current");
            cur.persisted_events.push(PersistedEvent {
                recorded_at: e.recorded_at,
                sequence: e.sequence as usize,
                event: serde_json::from_value(e.event).expect("Could not deserialize event"),
            });
        }
        if let Some(current) = current.take() {
            ret.push(E::try_from_events(current)?);
        }
        Ok((ret, false))
    }

    pub fn last_persisted(&self, n: usize) -> impl Iterator<Item = &PersistedEvent<T>> {
        let start = self.persisted_events.len() - n;
        self.persisted_events[start..].iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use es_entity_derive::{EsEntity, EsEvent};
    use uuid::Uuid;

    #[derive(Debug, serde::Serialize, serde::Deserialize, EsEvent)]
    #[es_event(id = Uuid)]
    enum DummyEntityEvent {
        Created(String),
    }

    #[derive(EsEntity)]
    struct DummyEntity {
        name: String,

        events: EntityEvents<DummyEntityEvent>,
    }

    impl TryFromEvents<DummyEntityEvent> for DummyEntity {
        fn try_from_events(events: EntityEvents<DummyEntityEvent>) -> Result<Self, EsEntityError> {
            let name = events
                .persisted()
                .map(|e| match &e.event {
                    DummyEntityEvent::Created(name) => name.clone(),
                })
                .next()
                .expect("Could not find name");
            Ok(Self { name, events })
        }
    }

    #[test]
    fn load_zero_events() {
        let generic_events = vec![];
        let res = EntityEvents::load_first::<DummyEntity>(generic_events);
        assert!(matches!(res, Err(EsEntityError::NotFound)));
    }

    #[test]
    fn load_first() {
        let generic_events = vec![GenericEvent {
            id: uuid::Uuid::new_v4(),
            sequence: 1,
            event: serde_json::to_value(DummyEntityEvent::Created("dummy-name".to_owned()))
                .expect("Could not serialize"),
            recorded_at: chrono::Utc::now(),
        }];
        let entity: DummyEntity = EntityEvents::load_first(generic_events).expect("Could not load");
        assert!(entity.name == "dummy-name");
    }

    #[test]
    fn load_n() {
        let generic_events = vec![
            GenericEvent {
                id: uuid::Uuid::new_v4(),
                sequence: 1,
                event: serde_json::to_value(DummyEntityEvent::Created("dummy-name".to_owned()))
                    .expect("Could not serialize"),
                recorded_at: chrono::Utc::now(),
            },
            GenericEvent {
                id: uuid::Uuid::new_v4(),
                sequence: 1,
                event: serde_json::to_value(DummyEntityEvent::Created("other-name".to_owned()))
                    .expect("Could not serialize"),
                recorded_at: chrono::Utc::now(),
            },
        ];
        let (entity, more): (Vec<DummyEntity>, _) =
            EntityEvents::load_n(generic_events, 2).expect("Could not load");
        assert!(!more);
        assert_eq!(entity.len(), 2);
    }
}
