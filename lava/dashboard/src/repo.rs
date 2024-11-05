use sqlx::PgPool;

use crate::{error::*, primitives::TimeRange, values::*};

#[derive(Clone)]
pub struct DashboardRepo {
    pool: PgPool,
}

impl DashboardRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, DashboardError> {
        Ok(self.pool.begin().await?)
    }

    pub async fn persist_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        range: TimeRange,
        values: &DashboardValues,
    ) -> Result<(), DashboardError> {
        let values = serde_json::to_value(values).expect("Could not serialize dashboard");
        sqlx::query!(
            r#"
            INSERT INTO dashboards (time_range, dashboard_json)
            VALUES ($1, $2)
            ON CONFLICT (time_range) DO UPDATE
            SET dashboard_json = $2
            "#,
            range as TimeRange,
            values
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn load_for_time_range(
        &self,
        range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        let row = sqlx::query!(
            r#" 
            SELECT dashboard_json
            FROM dashboards
            WHERE time_range = $1
            "#,
            range as TimeRange
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(row) = row {
            let values: DashboardValues = serde_json::from_value(row.dashboard_json)
                .expect("Could not de-serialize dashboard");
            if range.in_same_range(values.last_updated, chrono::Utc::now()) {
                return Ok(values);
            }
        }
        Ok(DashboardValues::new(range))
    }
}
