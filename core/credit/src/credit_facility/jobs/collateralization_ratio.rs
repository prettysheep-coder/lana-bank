use std::collections::HashSet;

use futures::StreamExt;

use job::{CurrentJob, Job, JobCompletion, JobInitializer, JobRunner, JobType};
use outbox::{EventSequence, Outbox, OutboxEventMarker};

use crate::{CoreCreditEvent, CreditFacility, CreditFacilityRepo, CreditLedger};

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct CollateralizationRatioData {
    sequence: EventSequence,
}

pub struct CollateralizationRatioRunner<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    outbox: Outbox<CoreCreditEvent>,
    repo: CreditFacilityRepo<E>,
    ledger: CreditLedger,
}

#[async_trait::async_trait]
impl<E> JobRunner for CollateralizationRatioRunner<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    async fn run(
        &self,
        mut current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        use CoreCreditEvent::*;

        let mut state = current_job
            .execution_state::<CollateralizationRatioData>()?
            .unwrap_or_default();

        let stream = self.outbox.listen_persisted(Some(state.sequence)).await?;

        let (ids, sequence) = stream
            .fold(
                (HashSet::new(), state.sequence),
                |(mut acc, _last_sequence), message| async move {
                    let id = message.payload.as_ref().and_then(|event| match event {
                        FacilityCollateralUpdated {
                            credit_facility_id, ..
                        } => Some(*credit_facility_id),
                        DisbursalSettled {
                            credit_facility_id, ..
                        } => Some(*credit_facility_id),
                        AccrualPosted {
                            credit_facility_id, ..
                        } => Some(*credit_facility_id),
                        FacilityRepaymentRecorded {
                            credit_facility_id, ..
                        } => Some(*credit_facility_id),
                        _ => None,
                    });

                    if let Some(id) = id {
                        acc.insert(id);
                    }

                    (acc, message.sequence)
                },
            )
            .await;

        let ids = ids.into_iter().collect::<Vec<_>>();
        let facilities = self.repo.find_all::<CreditFacility>(&ids).await?;

        let mut db = self.repo.begin_op().await?;

        for (_, mut facility) in facilities {
            let balance = self
                .ledger
                .get_credit_facility_balance(facility.account_ids)
                .await?;
            facility.record_collateralization_ratio(&balance);
            self.repo.update_in_op(&mut db, &mut facility).await?;
        }

        state.sequence = sequence;

        let mut db = db.into_tx();

        current_job
            .update_execution_state_in_tx(&mut db, &state)
            .await?;

        db.commit().await?;

        Ok(JobCompletion::RescheduleNow)
    }
}

pub struct CollateralizationRatioInitializer<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    outbox: Outbox<CoreCreditEvent>,
    repo: CreditFacilityRepo<E>,
    ledger: CreditLedger,
}

impl<E> CollateralizationRatioInitializer<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    pub fn new(
        outbox: &Outbox<CoreCreditEvent>,
        repo: &CreditFacilityRepo<E>,
        ledger: &CreditLedger,
    ) -> Self {
        Self {
            outbox: outbox.clone(),
            repo: repo.clone(),
            ledger: ledger.clone(),
        }
    }
}

const COLLATERALIZATION_RATIO: JobType = JobType::new("credit-facility-collateralization-ratio");
impl<E> JobInitializer for CollateralizationRatioInitializer<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        COLLATERALIZATION_RATIO
    }

    fn init(&self, _: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(CollateralizationRatioRunner {
            outbox: self.outbox.clone(),
            repo: self.repo.clone(),
            ledger: self.ledger.clone(),
        }))
    }
}
