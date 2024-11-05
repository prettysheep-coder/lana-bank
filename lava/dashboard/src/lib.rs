#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

// mod job;
mod error;
mod primitives;
mod repo;
mod values;

use sqlx::PgPool;

use lava_events::LavaEvent;

use error::*;
pub use primitives::*;
use repo::*;
pub use values::*;

type Outbox = outbox::Outbox<LavaEvent>;

pub struct Dashboard {
    _outbox: Outbox,
    repo: DashboardRepo,
}

impl Dashboard {
    pub fn new(pool: &PgPool, outbox: &Outbox) -> Self {
        Self {
            _outbox: outbox.clone(),
            repo: DashboardRepo::new(pool),
        }
    }

    pub async fn load_for_time_range(
        &self,
        range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        let res = self.repo.load_for_time_range(range).await?;
        Ok(res)
    }
}
