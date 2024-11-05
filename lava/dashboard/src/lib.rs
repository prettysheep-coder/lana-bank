#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod error;
mod job;
mod primitives;
mod repo;
mod values;

use sqlx::PgPool;

use audit::AuditSvc;
use authz::PermissionCheck;
use lava_events::LavaEvent;

use error::*;
use job::*;
pub use primitives::*;
use repo::*;
pub use values::*;

pub type Outbox = outbox::Outbox<LavaEvent>;

pub struct Dashboard<Perms>
where
    Perms: PermissionCheck,
{
    _outbox: Outbox,
    authz: Perms,
    repo: DashboardRepo,
}

impl<Perms> Dashboard<Perms>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<DashboardModuleAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<DashboardModuleObject>,
{
    pub async fn init(
        pool: &PgPool,
        authz: &Perms,
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
            authz: authz.clone(),
            repo,
        })
    }

    pub async fn load_for_time_range(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        self.authz
            .enforce_permission(
                sub,
                DashboardModuleObject::Dashboard,
                DashboardModuleAction::DASHBOARD_READ,
            )
            .await?;
        let res = self.repo.load_for_time_range(range).await?;
        Ok(res)
    }
}
