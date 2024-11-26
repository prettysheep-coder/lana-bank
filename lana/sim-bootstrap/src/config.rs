use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct BootstrapConfig {
    #[serde(default = "default_num_facilities")]
    num_facilities: u32,
}

fn default_num_facilities() -> u32 {
    4
}
