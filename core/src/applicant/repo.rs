use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    data_export::Export,
    entity::{EntityEvent, EntityEvents},
    primitives::CustomerId,
};

use super::error::ApplicantError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApplicantEvent {
    WebhookReceived {
        customer_id: CustomerId,
        event_type: String,
        webhook_data: Value,
        #[serde(with = "chrono::serde::ts_milliseconds")]
        timestamp: chrono::DateTime<Utc>,
    },
}

impl EntityEvent for ApplicantEvent {
    type EntityId = CustomerId;

    fn event_table_name() -> &'static str {
        "applicant_events"
    }
}

#[derive(Clone)]
pub struct ApplicantRepo {
    pool: PgPool,
    export: Export,
}

impl ApplicantRepo {
    pub fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }

    pub async fn persist_webhook(
        &self,
        customer_id: CustomerId,
        event_type: &str,
        webhook_data: Value,
    ) -> Result<(), ApplicantError> {
        let mut db = self.pool.begin().await?;
        self.persist_webhook_in_tx(&mut db, customer_id, event_type, webhook_data)
            .await?;
        db.commit().await?;
        Ok(())
    }

    pub async fn persist_webhook_in_tx(
        &self,
        db: &mut Transaction<'_, Postgres>,
        customer_id: CustomerId,
        event_type: &str,
        webhook_data: Value,
    ) -> Result<(), ApplicantError> {
        let event = ApplicantEvent::WebhookReceived {
            customer_id,
            event_type: event_type.to_string(),
            webhook_data,
            timestamp: Utc::now(),
        };

        let mut events = EntityEvents::init(customer_id, vec![event]);

        events.persist(db).await?;

        self.export
            .export_last(db, ApplicantEvent::event_table_name(), 1, &events)
            .await?;

        Ok(())
    }
}
