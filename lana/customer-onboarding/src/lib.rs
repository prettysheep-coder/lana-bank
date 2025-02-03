#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

pub mod config;
pub mod error;
mod job;
mod kratos_customer;
mod time;

use core_user::Role;
use lana_events::LanaEvent;
use rbac_types::{LanaAction, LanaObject, Subject};

use config::*;
use error::*;
use job::*;
use kratos_customer::KratosCustomer;

pub type Outbox = outbox::Outbox<LanaEvent>;
pub type Audit = audit::Audit<Subject, LanaObject, LanaAction>;
pub type Authorization = authz::Authorization<Audit, Role>;
pub type Deposits = deposit::CoreDeposit<Authorization, lana_events::LanaEvent>;

#[derive(Clone)]
pub struct CustomerOnboarding {
    _outbox: Outbox,
}

impl CustomerOnboarding {
    pub async fn init(
        jobs: &::job::Jobs,
        outbox: &Outbox,
        deposit: &Deposits,
        config: CustomerOnboardingConfig,
    ) -> Result<Self, CustomerOnboardingError> {
        let kratos_customer = KratosCustomer::init(config.kratos_customer);

        jobs.add_initializer_and_spawn_unique(
            CustomerOnboardingJobInitializer::new(outbox, kratos_customer),
            CustomerOnboardingJobConfig,
        )
        .await?;
        Ok(Self {
            _outbox: outbox.clone(),
        })
    }
}
