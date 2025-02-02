mod config;
mod error;

pub use config::KratosCustomerConfig;
pub use error::KratosCustomerError;

use ory_kratos_client::{
    apis::{configuration::Configuration, identity_api::create_identity},
    models::create_identity_body::CreateIdentityBody,
};

use core_customer::CustomerId;

#[derive(Clone)]
pub struct KratosCustomer {
    pub config: Configuration,
}

impl KratosCustomer {
    pub fn init(config: KratosCustomerConfig) -> Self {
        Self {
            config: Configuration {
                base_path: config.kratos_customer_url.clone(),
                ..Default::default()
            },
        }
    }

    pub async fn create_customer(
        &self,
        user_id: CustomerId,
        email: String,
    ) -> Result<(), KratosCustomerError> {
        let identity = CreateIdentityBody {
            schema_id: "email".to_string(),
            traits: serde_json::json!({
                "email": email,
                "customer_id": user_id.to_string(),
            }),
            credentials: None,
            metadata_admin: None,
            metadata_public: None,
            recovery_addresses: None,
            state: None,
            verifiable_addresses: None,
        };

        create_identity(&self.config, Some(&identity)).await?;

        Ok(())
    }
}
