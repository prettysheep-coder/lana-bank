use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BootstrapConfig {
    #[serde(default = "default_num_facilities")]
    pub num_facilities: u32,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            num_facilities: default_num_facilities(),
        }
    }
}

fn default_num_facilities() -> u32 {
    4
}
