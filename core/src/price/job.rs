use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    data_export::{Export, ExportPriceData},
    job::*,
    price::Price,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ExportPriceJobConfig {
    pub job_interval: ExportPriceInterval,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExportPriceInterval {
    EveryMinute,
}

impl ExportPriceInterval {
    fn timestamp(&self) -> DateTime<Utc> {
        match self {
            ExportPriceInterval::EveryMinute => {
                let now = Utc::now();
                now + chrono::Duration::minutes(1)
            }
        }
    }
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

const PRICE_EXPORT_JOB: JobType = JobType::new("price-export");
impl JobInitializer for ExportPriceInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        PRICE_EXPORT_JOB
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
            self.config.job_interval.timestamp(),
        ))
    }
}
