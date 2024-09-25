use gcp_bigquery_client::yup_oauth2::ServiceAccountKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DocsConfig {
    #[serde(skip)]
    pub gcp_project: String,
    #[serde(skip)]
    pub sa_creds_base64: String,
    #[serde(skip)]
    service_account_key: Option<ServiceAccountKey>,

    pub gcp_location: String,
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    pub docs_root_folder: String,
    #[serde(default)]
    pub download_link_duration: u32,
}
