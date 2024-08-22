mod job;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{
    entity::{EntityEvent, EntityEvents},
    job::{error::JobError, JobRegistry, Jobs},
    primitives::JobId,
};

use job::{DataExportConfig, DataExportInitializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    id: uuid::Uuid,
    event_type: String,
    event: serde_json::Value,
    sequence: usize,
    recorded_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Export {
    jobs: Option<Jobs>,
}

impl Export {
    pub fn new(registry: &mut JobRegistry) -> Self {
        registry.add_initializer(DataExportInitializer::new());
        Self { jobs: None }
    }

    pub fn set_jobs(&mut self, jobs: &Jobs) {
        self.jobs = Some(jobs.clone());
    }

    fn jobs(&self) -> &Jobs {
        self.jobs.as_ref().expect("Jobs must already be set")
    }

    pub async fn export_all<T: EntityEvent + 'static>(
        &self,
        db: &mut Transaction<'_, Postgres>,
        events: &EntityEvents<T>,
    ) -> Result<(), JobError> {
        let n_events = events.len_persisted();
        self.export_last(db, n_events, events).await
    }

    pub async fn export_last<T: EntityEvent + 'static>(
        &self,
        db: &mut Transaction<'_, Postgres>,
        last: usize,
        events: &EntityEvents<T>,
    ) -> Result<(), JobError> {
        let id: uuid::Uuid = events.entity_id.into();
        let recorded_at = events
            .latest_event_persisted_at
            .expect("No events persisted");
        for (sequence, event) in events.last_persisted(last) {
            let event = serde_json::to_value(event).expect("Couldn't serialize event");
            let event_type = event
                .get("type")
                .expect("Event must have a type")
                .as_str()
                .expect("Type must be a string")
                .to_string();
            let data = ExportData {
                id,
                event,
                event_type,
                sequence,
                recorded_at,
            };
            self.jobs()
                .create_and_spawn_job::<DataExportInitializer, _>(
                    db,
                    JobId::new(),
                    format!("export:{}:{}", id, sequence),
                    DataExportConfig { data },
                )
                .await?;
        }
        Ok(())
    }
}
