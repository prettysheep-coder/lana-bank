use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct StorageConfig {
    #[serde(default)]
    pub root_folder: String,
    #[serde(default)]
    pub bucket_name: String,
}

impl StorageConfig {
    /// This function only needs to be run in dev mode
    ///
    /// in production, the value is set directly through lava.yml
    pub fn new_dev_mode(name_prefix: String) -> StorageConfig {
        Self {
            bucket_name: format!("{}-volcano-documents", name_prefix),
            root_folder: name_prefix,
        }
    }
}
