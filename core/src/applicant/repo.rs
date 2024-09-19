use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    data_export::{Export, ExportSumsubApplicantData, SumsubContentType},
    entity::EntityEvent,
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
        webhook_data: Value,
    ) -> Result<(), ApplicantError> {
        let mut db = self.pool.begin().await?;
        self.persist_webhook_in_tx(&mut db, customer_id, webhook_data)
            .await?;
        db.commit().await?;
        Ok(())
    }

    pub async fn persist_webhook_in_tx(
        &self,
        db: &mut Transaction<'_, Postgres>,
        customer_id: CustomerId,
        webhook_data: Value,
    ) -> Result<(), ApplicantError> {
        sqlx::query(
            r#"
            INSERT INTO sumsub_callbacks (id, content)
            VALUES ($1, $2)
            "#,
        )
        .bind(customer_id)
        .bind(&webhook_data.clone())
        .execute(&mut **db)
        .await?;

        // Prepare the data for export to BigQuery
        let export_data = ExportSumsubApplicantData {
            customer_id,
            content: serde_json::to_string(&webhook_data)?,
            content_type: SumsubContentType::Webhook,
            uploaded_at: Utc::now(),
        };

        // Export the data to BigQuery
        self.export
            .export_sum_sub_applicant_data(export_data)
            .await?;

        Ok(())
    }
}
