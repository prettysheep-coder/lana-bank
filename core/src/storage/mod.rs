pub mod config;
mod error;

pub use error::StorageError;

use cloud_storage::{ListRequest, Object};
use config::StorageConfig;
use futures_util::TryStreamExt;

use crate::report::ReportLocationInCloud;

const LINK_DURATION_IN_SECS: u32 = 60 * 5;

#[derive(Clone)]
pub struct Storage {
    // TODO: make private
    pub config: StorageConfig,
}

impl Storage {
    pub fn new(config: &StorageConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn upload(&self, file: Vec<u8>, path_in_bucket: &str) -> Result<(), StorageError> {
        Object::create(
            &self.config.bucket_name,
            file,
            path_in_bucket,
            "application/xml",
        )
        .await?;

        Ok(())
    }

    pub async fn generate_download_link(
        &self,
        location: &ReportLocationInCloud,
    ) -> Result<String, StorageError> {
        Ok(Object::read(&location.bucket, &location.path_in_bucket)
            .await?
            .download_url(LINK_DURATION_IN_SECS)?)
    }

    pub async fn list(&self, prefix: String) -> anyhow::Result<Vec<String>> {
        println!("bucket name: {}", self.config.bucket_name);

        let mut filenames = Vec::new();
        let stream = Object::list(
            &self.config.bucket_name,
            ListRequest {
                prefix: Some(prefix),
                ..Default::default()
            },
        )
        .await?;

        let mut stream = Box::pin(stream.into_stream());

        while let Some(result) = stream.try_next().await? {
            for item in result.items {
                filenames.push(item.name);
            }
        }

        Ok(filenames)
    }
}
