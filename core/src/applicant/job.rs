use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use std::borrow::Cow;

use crate::{data_export::ExportData, job::*, primitives::CustomerId};

use super::SumsubClient;

use crate::data_export::cala::CalaClient;

#[derive(Clone, Deserialize, Serialize)]
pub struct SumsubExportConfig {
    pub customer_id: CustomerId,
}

pub struct SumsubExportInitializer {
    pub(super) cala_url: String,
    pub(super) table_name: Cow<'static, str>,
    pub(super) sumsub_client: SumsubClient,
}

impl SumsubExportInitializer {
    pub fn new(
        cala_url: String,
        table_name: Cow<'static, str>,
        sumsub_client: SumsubClient,
    ) -> Self {
        Self {
            cala_url,
            table_name,
            sumsub_client,
        }
    }
}

const SUMSUB_EXPORT_JOB: JobType = JobType::new("sumsub-export");
impl JobInitializer for SumsubExportInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        SUMSUB_EXPORT_JOB
    }

    fn init(&self, job: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(SumsubExportJobRunner {
            config: job.config()?,
            cala_url: self.cala_url.clone(),
            table_name: self.table_name.clone(),
            sumsub_client: self.sumsub_client.clone(),
        }))
    }
}

pub struct SumsubExportJobRunner {
    config: SumsubExportConfig,
    cala_url: String,
    table_name: Cow<'static, str>,
    sumsub_client: SumsubClient,
}

#[async_trait]
impl JobRunner for SumsubExportJobRunner {
    #[tracing::instrument(name = "lava.sumsub_export.job.run", skip_all, fields(insert_id), err)]
    async fn run(&self, _: CurrentJob) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        let customer_id = self.config.customer_id;

        let client = reqwest::Client::new();
        let res = self
            .sumsub_client
            .get_applicant_details(&client, customer_id)
            .await?;

        let _info = res.info;

        let dummy_exported_data = ExportData {
            id: self.config.customer_id.into(),
            event_type: "dummy".to_string(),
            event: "dummy".to_string(),
            sequence: 0,
            recorded_at: chrono::Utc::now(),
        };

        let cala = CalaClient::new(self.cala_url.clone());
        cala.insert_bq_row(&self.table_name, &dummy_exported_data)
            .await?;
        Ok(JobCompletion::Complete)
    }
}
