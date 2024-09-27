use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ReportConfig {
    #[serde(default)]
    pub dataform_repo: String,
    #[serde(default)]
    pub dataform_output_dataset: String,
    #[serde(default)]
    pub dataform_release_config: String,
}

impl ReportConfig {
    pub fn new_dev_mode(name_prefix: String) -> ReportConfig {
        Self {
            dataform_repo: format!("{}-repo", name_prefix),
            dataform_output_dataset: format!("dataform_{}", name_prefix),
            dataform_release_config: format!("{}-release", name_prefix),
        }
    }
}
