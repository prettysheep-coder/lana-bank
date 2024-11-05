use sqlx::PgPool;

use crate::{error::*, primitives::TimeRange, values::*};

pub struct DashboardRepo {
    _pool: PgPool,
}

impl DashboardRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            _pool: pool.clone(),
        }
    }

    pub async fn load_for_time_range(
        &self,
        _range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        let res = DashboardValues {
            active_facilities: 1,
            pending_facilities: 0,
        };
        Ok(res)
    }
}
