use ory_kratos_client::apis::identity_api::CreateIdentityError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KratosCustomerError {
    #[error("KratosCustomerError - OryKratosCustomerApiCreateIdentityError: {0}")]
    KratosCustomerApiCreateIdentityError(
        #[from] ory_kratos_client::apis::Error<CreateIdentityError>,
    ),
}
