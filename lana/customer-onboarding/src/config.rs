use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerOnboardingConfig {
    pub kratos_customer: super::kratos_customer::KratosCustomerConfig,
}
