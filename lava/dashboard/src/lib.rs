#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod error;
mod job;
mod primitives;
mod repo;
mod values;

use sqlx::PgPool;

use lava_events::LavaEvent;

use error::*;
use job::*;
pub use primitives::*;
use repo::*;
pub use values::*;

pub type Outbox = outbox::Outbox<LavaEvent>;

pub struct Dashboard {
    _outbox: Outbox,
    repo: DashboardRepo,
}

impl Dashboard {
    pub async fn init(
        pool: &PgPool,
        jobs: &::job::Jobs,
        outbox: &Outbox,
    ) -> Result<Self, DashboardError> {
        let repo = DashboardRepo::new(pool);
        jobs.add_initializer_and_spawn_unique(
            DashboardProjectionJobInitializer::new(outbox, &repo),
            DashboardProjectionJobConfig,
        )
        .await?;
        Ok(Self {
            _outbox: outbox.clone(),
            repo,
        })
    }

    pub async fn load_for_time_range(
        &self,
        range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        let res = self.repo.load_for_time_range(range).await?;
        Ok(res)
    }
}
