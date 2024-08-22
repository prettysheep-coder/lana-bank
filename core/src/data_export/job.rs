use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::job::*;

use super::ExportData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExportConfig {
    pub data: ExportData,
}

pub struct DataExportInitializer {}

impl DataExportInitializer {
    pub fn new() -> Self {
        Self {}
    }
}

const DATA_EXPORT_JOB: JobType = JobType::new("data-export");
impl JobInitializer for DataExportInitializer {
    fn job_type() -> JobType
    where
        Self: Sized,
    {
        DATA_EXPORT_JOB
    }

    fn init(&self, job: &Job) -> Result<Box<dyn JobRunner>, Box<dyn std::error::Error>> {
        Ok(Box::new(DataExportJobRunner {
            config: job.config()?,
        }))
    }
}

pub struct DataExportJobRunner {
    config: DataExportConfig,
}

#[async_trait]
impl JobRunner for DataExportJobRunner {
    async fn run(
        &self,
        _current_job: CurrentJob,
    ) -> Result<JobCompletion, Box<dyn std::error::Error>> {
        println!("export: {:?}", self.config.data);
        Ok(JobCompletion::Complete)
    }
}
