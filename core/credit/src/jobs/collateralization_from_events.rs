use futures::StreamExt;

use std::collections::HashSet;

use audit::AuditSvc;
use authz::PermissionCheck;
use job::*;
use outbox::{EventSequence, Outbox, OutboxEventMarker};

use crate::{
    credit_facility::{CreditFacility, CreditFacilityRepo},
    event::CoreCreditEvent,
    ledger::CreditLedger,
    primitives::*,
};

#[derive(serde::Serialize)]
pub struct CreditFacilityCollateralizationFromEventsJobConfig<Perms, E> {
    _phantom: std::marker::PhantomData<(Perms, E)>,
}
impl<Perms, E> CreditFacilityCollateralizationFromEventsJobConfig<Perms, E> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Perms, E> Default for CreditFacilityCollateralizationFromEventsJobConfig<Perms, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Perms, E> JobConfig for CreditFacilityCollateralizationFromEventsJobConfig<Perms, E>
where
    Perms: PermissionCheck,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Action: From<CoreCreditAction>,
    <<Perms as PermissionCheck>::Audit as AuditSvc>::Object: From<CoreCreditObject>,
    E: OutboxEventMarker<CoreCreditEvent>,
{
    type Initializer = CreditFacilityCollateralizationFromEventsInitializer<Perms, E>;
}

pub struct CreditFacilityCollateralizationFromEventsInitializer<Perms, E>
where
    Perms: PermissionCheck,
    E: OutboxEventMarker<CoreCreditEvent>,
{
    audit: Perms::Audit,
    outbox: Outbox<E>,
    repo: CreditFacilityRepo<E>,
    ledger: CreditLedger,
}

impl<Perms, E> CreditFacilityCollateralizationFromEventsInitializer<Perms, E>
where
    Perms: PermissionCheck,
    E: OutboxEventMarker<CoreCreditEvent>,
{
    pub fn new(
        outbox: &Outbox<E>,
        repo: &CreditFacilityRepo<E>,
        ledger: &CreditLedger,
        audit: &Perms::Audit,
    ) -> Self {
        Self {
            outbox: outbox.clone(),
            repo: repo.clone(),
            ledger: ledger.clone(),
            audit: audit.clone(),
        }
    }
}

const CREDIT_FACILITY_COLLATERALIZATION_FROM_EVENTS_JOB: JobType =
    JobType::new("credit-facility-collateralization-from-events");
impl<Perms, E> JobInitializer for CreditFacilityCollateralizationFromEventsInitializer<Perms, E>
where
    Perms: PermissionCheck,
    E: OutboxEventMarker<CoreCreditEvent>,
{
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        CREDIT_FACILITY_COLLATERALIZATION_FROM_EVENTS_JOB
    }

    fn init(&self, _: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(CreditFacilityCollateralizationFromEventsRunner {
            outbox: self.outbox.clone(),
            repo: self.repo.clone(),
            ledger: self.ledger.clone(),
        }))
    }
}

// TODO: reproduce 'collateralization_ratio' test from old credit facility

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct CreditFacilityCollateralizationFromEventsData {
    sequence: EventSequence,
}

pub struct CreditFacilityCollateralizationFromEventsRunner<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    outbox: Outbox<E>,
    repo: CreditFacilityRepo<E>,
    ledger: CreditLedger,
}

#[async_trait::async_trait]
impl<E> JobRunner for CreditFacilityCollateralizationFromEventsRunner<E>
where
    E: OutboxEventMarker<CoreCreditEvent>,
{
    async fn run(
        &self,
        mut current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        unimplemented!()
        // use CoreCreditEvent::*;

        // let mut state = current_job
        //     .execution_state::<CollateralizationRatioData>()?
        //     .unwrap_or_default();

        // let stream = self.outbox.listen_persisted(Some(state.sequence)).await?;

        // let (ids, sequence) = stream
        //     .fold(
        //         (HashSet::new(), state.sequence),
        //         |(mut acc, _last_sequence), message| async move {
        //             let id = message.payload.as_ref().and_then(|event| match event {
        //                 FacilityCollateralUpdated {
        //                     credit_facility_id, ..
        //                 } => Some(*credit_facility_id),
        //                 DisbursalSettled {
        //                     credit_facility_id, ..
        //                 } => Some(*credit_facility_id),
        //                 AccrualPosted {
        //                     credit_facility_id, ..
        //                 } => Some(*credit_facility_id),
        //                 FacilityRepaymentRecorded {
        //                     credit_facility_id, ..
        //                 } => Some(*credit_facility_id),
        //                 _ => None,
        //             });

        //             if let Some(id) = id {
        //                 acc.insert(id);
        //             }

        //             (acc, message.sequence)
        //         },
        //     )
        //     .await;

        // let ids = ids.into_iter().collect::<Vec<_>>();
        // let facilities = self.repo.find_all::<CreditFacility>(&ids).await?;

        // let mut db = self.repo.begin_op().await?;

        // for (_, mut facility) in facilities {
        //     let balance = self
        //         .ledger
        //         .get_credit_facility_balance(facility.account_ids)
        //         .await?;
        //     facility.record_collateralization_ratio(&balance);
        //     self.repo.update_in_op(&mut db, &mut facility).await?;
        // }

        // state.sequence = sequence;

        // let mut db = db.into_tx();

        // current_job
        //     .update_execution_state_in_tx(&mut db, &state)
        //     .await?;

        // db.commit().await?;

        // Ok(JobCompletion::RescheduleNow)
    }
}
