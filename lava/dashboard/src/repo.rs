use sqlx::PgPool;

use crate::{error::*, primitives::TimeRange, values::*};

pub struct DashboardRepo {
    pool: PgPool,
}

impl DashboardRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
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
            let values = serde_json::from_value(row.dashboard_json)
                .expect("Could not de-seralize dashboard");
            Ok(values)
        } else {
            Ok(DashboardValues::default())
        }
    }
}
