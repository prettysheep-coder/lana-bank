use crate::primitives::CustomerId;

pub fn get_customer_link_sumsub(customer_id: CustomerId) -> String {
    format!(
        "https://cockpit.sumsub.com/checkus#/applicants/individual?limit=10&page=0&searchQuery={}",
        customer_id
    )
}
