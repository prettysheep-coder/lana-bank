use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KratosCustomerConfig {
    #[serde(default = "default_kratos_customer_url")]
    pub kratos_customer_url: String,
}

impl Default for KratosCustomerConfig {
    fn default() -> Self {
        Self {
            kratos_customer_url: default_kratos_customer_url(),
        }
    }
}

fn default_kratos_customer_url() -> String {
    "http://localhost:4434".to_string()
}
