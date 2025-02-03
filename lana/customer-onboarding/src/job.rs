use async_trait::async_trait;
use core_customer::CoreCustomerEvent;
use futures::StreamExt;

use job::*;
use lana_events::LanaEvent;

use super::{kratos_customer::KratosCustomer, Deposits, Outbox};

#[derive(serde::Serialize)]
pub struct CustomerOnboardingJobConfig;
impl JobConfig for CustomerOnboardingJobConfig {
    type Initializer = CustomerOnboardingJobInitializer;
}

pub struct CustomerOnboardingJobInitializer {
    outbox: Outbox,
    deposit: Deposits,
    kratos_customer: KratosCustomer,
}

impl CustomerOnboardingJobInitializer {
    pub fn new(outbox: &Outbox, deposit: &Deposits, kratos_customer: KratosCustomer) -> Self {
        Self {
            outbox: outbox.clone(),
            deposit: deposit.clone(),
            kratos_customer,
        }
    }
}

const CUSTOMER_ONBOARDING_JOB: JobType = JobType::new("customer-onboarding");

impl JobInitializer for CustomerOnboardingJobInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        CUSTOMER_ONBOARDING_JOB
    }

    fn init(&self, _: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(CustomerOnboardingJobRunner {
            outbox: self.outbox.clone(),
            kratos_customer: self.kratos_customer.clone(),
            deposit: self.deposit.clone(),
        }))
    }

    fn retry_on_error_settings() -> RetrySettings
    where
        Self: Sized,
    {
        RetrySettings::repeat_indefinitely()
    }
}

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct CustomerOnboardingJobData {
    sequence: outbox::EventSequence,
}

pub struct CustomerOnboardingJobRunner {
    outbox: Outbox,
    kratos_customer: KratosCustomer,
    deposit: Deposits,
}
#[async_trait]
impl JobRunner for CustomerOnboardingJobRunner {
    async fn run(
        &self,
        current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        let state = current_job
            .execution_state::<CustomerOnboardingJobData>()?
            .unwrap_or_default();
        let mut stream = self.outbox.listen_persisted(Some(state.sequence)).await?;

        while let Some(message) = stream.next().await {
            if let Some(LanaEvent::Customer(CoreCustomerEvent::CustomerCreated { id, email })) =
                &message.payload
            {
                // how to ensure atomicity
                let account_name = &format!("Deposit Account for Customer {}", id);
                self.deposit
                    .create_account(*id, account_name, account_name)
                    .await?;
                self.kratos_customer
                    .create_customer(*id, email.clone())
                    .await?;
            }
        }

        let now = crate::time::now();
        Ok(JobCompletion::RescheduleAt(now))
    }
}
