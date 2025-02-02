use serde::{Deserialize, Serialize};

use super::kratos_customer::KratosCustomerConfig;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerOnboardingConfig {
    pub kratos_customer: KratosCustomerConfig,
}
