use async_trait::async_trait;
use futures::StreamExt;

use job::*;

use crate::{primitives::*, repo::DashboardRepo, values::*, Outbox};

#[derive(serde::Serialize)]
pub struct DashboardProjectionJobConfig;
impl JobConfig for DashboardProjectionJobConfig {
    type Initializer = DashboardProjectionJobInitializer;
}

pub struct DashboardProjectionJobInitializer {
    outbox: Outbox,
    repo: DashboardRepo,
}

impl DashboardProjectionJobInitializer {
    pub fn new(outbox: &Outbox, repo: &DashboardRepo) -> Self {
        Self {
            repo: repo.clone(),
            outbox: outbox.clone(),
        }
    }
}

const DASHBOARD_PROJECTION_JOB: JobType = JobType::new("dashboard-projection");
impl JobInitializer for DashboardProjectionJobInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        DASHBOARD_PROJECTION_JOB
    }

    fn init(&self, _: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(DashboardProjectionJobRunner {
            outbox: self.outbox.clone(),
            repo: self.repo.clone(),
        }))
    }

    fn retry_on_error_settings() -> RetrySettings
    where
        Self: Sized,
    {
        RetrySettings::repeat_indefinitely()
    }
}

#[derive(Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
struct DashboardProjectionJobData {
    sequence: outbox::EventSequence,
}

pub struct DashboardProjectionJobRunner {
    outbox: Outbox,
    repo: DashboardRepo,
}
#[async_trait]
impl JobRunner for DashboardProjectionJobRunner {
    #[allow(clippy::single_match)]
    async fn run(
        &self,
        mut current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        let mut state = current_job
            .execution_state::<DashboardProjectionJobData>()?
            .unwrap_or_default();
        let mut stream = self.outbox.listen_persisted(Some(state.sequence)).await?;
        let mut dashboard = DashboardValues::default();

        while let Some(message) = stream.next().await {
            if let Some(payload) = &message.payload {
                if dashboard.process_event(payload) {
                    let mut db = self.repo.begin().await?;
                    self.repo
                        .persist_in_tx(&mut db, TimeRange::ThisQuarter, &dashboard)
                        .await?;
                    state.sequence = message.sequence;
                    current_job
                        .update_execution_state_in_tx(&mut db, state)
                        .await?;
                    db.commit().await?;
                }
            }
        }

        Ok(JobCompletion::RescheduleAt(chrono::Utc::now()))
    }
}
