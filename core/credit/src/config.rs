use serde::{Deserialize, Serialize};

use crate::terms::CVLPct;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreditFacilityConfig {
    #[serde(default = "default_upgrade_buffer_cvl_pct")]
    pub upgrade_buffer_cvl_pct: CVLPct,
    #[serde(default)]
    pub sumsub_enabled: bool,
}

impl Default for CreditFacilityConfig {
    fn default() -> Self {
        CreditFacilityConfig {
            upgrade_buffer_cvl_pct: default_upgrade_buffer_cvl_pct(),
            sumsub_enabled: false,
        }
    }
}

fn default_upgrade_buffer_cvl_pct() -> CVLPct {
    CVLPct::new(5)
}
