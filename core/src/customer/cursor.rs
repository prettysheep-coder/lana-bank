use serde::{Deserialize, Serialize};

use super::{Customer, CustomerId};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerByEmailCursor {
    pub email: String,
    pub id: CustomerId,
}

impl From<&Customer> for CustomerByEmailCursor {
    fn from(values: &Customer) -> Self {
        Self {
            email: values.email.clone(),
            id: values.id,
        }
    }
}
