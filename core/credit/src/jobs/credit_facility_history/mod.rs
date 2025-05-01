mod entry;
mod error;
mod repo;
mod value;

use futures::StreamExt;

use job::{CurrentJob, Job, JobCompletion, JobInitializer, JobRunner, JobType};
use outbox::{EventSequence, Outbox};

use crate::CoreCreditEvent;

use repo::*;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct HistoryProjectionJobData {
    sequence: EventSequence,
}

pub struct HistoryProjectionJobRunner {
    outbox: Outbox<CoreCreditEvent>,
    repo: HistoryRepo,
}

#[async_trait::async_trait]
impl JobRunner for HistoryProjectionJobRunner {
    async fn run(
        &self,
        mut current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        use CoreCreditEvent::*;

        let mut state = current_job
            .execution_state::<HistoryProjectionJobData>()?
            .unwrap_or_default();

        let mut stream = self.outbox.listen_persisted(Some(state.sequence)).await?;

        while let Some(message) = stream.next().await {
            if let Some(event) = &message.payload {
                let id = match event {
                    FacilityCreated { id, .. }
                    | FacilityApproved { id }
                    | FacilityActivated { id, .. }
                    | FacilityCompleted { id, .. }
                    | FacilityRepaymentRecorded {
                        credit_facility_id: id,
                        ..
                    }
                    | FacilityCollateralUpdated {
                        credit_facility_id: id,
                        ..
                    }
                    | FacilityCollateralizationChanged { id, .. }
                    | DisbursalSettled {
                        credit_facility_id: id,
                        ..
                    }
                    | AccrualPosted {
                        credit_facility_id: id,
                        ..
                    }
                    | ObligationCreated {
                        credit_facility_id: id,
                        ..
                    }
                    | ObligationDue {
                        credit_facility_id: id,
                        ..
                    } => *id,
                };

                let mut db = self.repo.begin().await?;

                let mut history = self.repo.load(id).await?;
                history.process_event(event);
                self.repo.persist_in_tx(&mut db, id, history).await?;

                state.sequence = message.sequence;
                current_job
                    .update_execution_state_in_tx(&mut db, &state)
                    .await?;

                db.commit().await?;
            }
        }

        Ok(JobCompletion::RescheduleNow)
    }
}

pub struct HistoryProjectionInitializer {
    outbox: Outbox<CoreCreditEvent>,
    repo: HistoryRepo,
}

impl HistoryProjectionInitializer {
    pub fn new(outbox: &Outbox<CoreCreditEvent>, repo: &HistoryRepo) -> Self {
        Self {
            outbox: outbox.clone(),
            repo: repo.clone(),
        }
    }
}

const HISTORY_PROJECTION: JobType = JobType::new("credit-facility-history-projection");
impl JobInitializer for HistoryProjectionInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        HISTORY_PROJECTION
    }

    fn init(&self, _: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(HistoryProjectionJobRunner {
            outbox: self.outbox.clone(),
            repo: self.repo.clone(),
        }))
    }
}
