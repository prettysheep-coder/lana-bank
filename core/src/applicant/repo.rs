use crate::primitives::CustomerId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use sqlx::{PgPool, Postgres, Row, Transaction};

use super::error::ApplicantError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApplicantEvent {
    WebhookReceived {
        customer_id: CustomerId,
        webhook_data: serde_json::Value,
        #[serde(with = "chrono::serde::ts_milliseconds")]
        timestamp: chrono::DateTime<Utc>,
    },
}

#[derive(Clone)]
pub struct ApplicantRepo {
    pool: PgPool,
}

impl ApplicantRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn persist_webhook(
        &self,
        db: &mut Transaction<'_, Postgres>,
        customer_id: CustomerId,
        webhook_data: serde_json::Value,
    ) -> Result<i64, ApplicantError> {
        let row = sqlx::query(
            r#"
            INSERT INTO sumsub_callbacks (customer_id, content)
            VALUES ($1, $2)
            RETURNING id
            "#,
        )
        .bind(customer_id)
        .bind(webhook_data)
        .fetch_one(&mut **db)
        .await?;

        let id: i64 = row.try_get("id")?;
        Ok(id)
    }

    pub async fn fetch_one(&self, id: i64) -> Result<ApplicantEvent, ApplicantError> {
        let row = sqlx::query(
            r#"
            SELECT customer_id, content, recorded_at
            FROM sumsub_callbacks
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        let customer_id: CustomerId = row.try_get("customer_id")?;
        let webhook_data: serde_json::Value = row.try_get("content")?;
        let timestamp: chrono::DateTime<Utc> = row.try_get("recorded_at")?;

        Ok(ApplicantEvent::WebhookReceived {
            customer_id,
            webhook_data,
            timestamp,
        })
    }
}
