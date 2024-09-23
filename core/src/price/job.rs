use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    data_export::{Export, ExportPriceData},
    job::*,
    price::Price,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ExportPriceJobConfig {
    pub job_interval: Duration,
}

pub struct ExportPriceInitializer {
    price: Price,
    export: Export,
}

impl ExportPriceInitializer {
    pub fn new(price: &Price, export: &Export) -> Self {
        Self {
            price: price.clone(),
            export: export.clone(),
        }
    }
}

const FETCH_PRICE_JOB: JobType = JobType::new("fetch-price");
impl JobInitializer for ExportPriceInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        FETCH_PRICE_JOB
    }

    fn init(&self, job: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(ExportPriceJobRunner {
            config: job.config()?,
            price: self.price.clone(),
            export: self.export.clone(),
        }))
    }
}

pub struct ExportPriceJobRunner {
    config: ExportPriceJobConfig,
    price: Price,
    export: Export,
}

#[async_trait]
impl JobRunner for ExportPriceJobRunner {
    async fn run(&self, _: CurrentJob) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        let price = self.price.usd_cents_per_btc().await?;
        self.export
            .export_price_data(ExportPriceData {
                usd_cents_per_btc: price,
                uploaded_at: Utc::now(),
            })
            .await?;

        Ok(JobCompletion::RescheduleAt(
            Utc::now() + self.config.job_interval,
        ))
    }
}
