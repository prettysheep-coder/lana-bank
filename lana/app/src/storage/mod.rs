pub mod config;
mod error;

pub use error::StorageError;

// use cloud_storage::{ListRequest, Object};
use config::StorageConfig;
use futures::TryStreamExt;
use google_cloud_storage::{
    client::{google_cloud_auth::credentials::CredentialsFile, Client, ClientConfig},
    http::objects::{
        delete::DeleteObjectRequest,
        upload::{Media, UploadObjectRequest},
    },
    sign::SignedURLOptions,
};

const LINK_DURATION_IN_SECS: u32 = 60 * 5;

#[derive(Debug, Clone)]
pub struct LocationInCloud<'a> {
    pub bucket: &'a str,
    pub path_in_bucket: &'a str,
}

#[derive(Clone)]
pub struct Storage {
    config: StorageConfig,
    client: Client,
}

impl Storage {
    pub async fn new(config: &StorageConfig) -> Result<Self, StorageError> {
        let creds = if let Some(creds) = config.service_account {
            CredentialsFile::new_from_str(creds.get_json_creds().as_ref()).await?;
        } else {
            CredentialsFile::new_from_env_var("GOOGLE_APPLICATION_CREDENTIALS").await?
        };

        let client_config = ClientConfig::default().with_credentials(creds).await?;
        let client = Client::new(client_config);

        Ok(Self {
            config: config.clone(),
            client,
        })
    }

    pub fn bucket_name(&self) -> String {
        self.config.bucket_name.clone()
    }

    fn path_with_prefix(&self, path: &str) -> String {
        format!("{}/{}", self.config.root_folder, path)
    }

    pub async fn upload(
        &self,
        file: Vec<u8>,
        path_in_bucket: &str,
        mime_type: &str,
    ) -> Result<(), StorageError> {
        let media = Media::new(self.path_with_prefix(path_in_bucket));

        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket_name(),
                    ..Default::default()
                },
                file,
                &google_cloud_storage::http::objects::upload::UploadType::Simple(media),
            )
            .await?;

        Ok(())
    }

    pub async fn remove(&self, location: LocationInCloud<'_>) -> Result<(), StorageError> {
        self.client
            .delete_object(&DeleteObjectRequest {
                bucket: self.bucket_name(),
                object: self.path_with_prefix(location.path_in_bucket),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn generate_download_link(
        &self,
        location: impl Into<LocationInCloud<'_>>,
    ) -> Result<String, StorageError> {
        let location = location.into();

        let url = self
            .client
            .signed_url(
                &self.bucket_name(),
                &self.path_with_prefix(location.path_in_bucket),
                None,
                None,
                SignedURLOptions::default(),
            )
            .await?;

        Ok(url)
    }
}
