use async_graphql::{SimpleObject, Union, ID};

use crate::{
    customer::Customers,
    server::{
        admin::graphql::user::User,
        shared_graphql::{customer::Customer, primitives::Timestamp},
    },
    user::Users,
};

#[derive(SimpleObject)]
pub struct System {
    name: String,
}

#[derive(Union)]
enum Subject {
    User(User),
    Customer(Customer),
    System(System),
}

#[derive(SimpleObject)]
pub struct AuditEntry {
    pub id: ID,
    subject: Subject,
    object: String,
    action: String,
    authorized: bool,
    created_at: Timestamp,
}

impl AuditEntry {
    pub async fn from_async(
        audit_log: crate::audit::AuditEntry,
        users: &Users,
        customers: &Customers,
    ) -> Self {
        let subject = match audit_log.subject {
            crate::primitives::Subject::User(id) => {
                let user = users.find_by_id_internal(id).await;
                let user = match user {
                    Ok(Some(user)) => user,
                    _ => {
                        return Self {
                            id: audit_log.id.0.into(),
                            subject: Subject::System(System {
                                name: "Not found".to_string(),
                            }),
                            object: audit_log.object.as_ref().into(),
                            action: audit_log.action.as_ref().into(),
                            authorized: audit_log.authorized,
                            created_at: audit_log.created_at.into(),
                        }
                    }
                };
                Subject::User(User::from(user))
            }
            crate::primitives::Subject::Customer(id) => {
                let customer = customers.find_by_id_internal(id).await.unwrap().unwrap();
                Subject::Customer(Customer::from(customer))
            }
            crate::primitives::Subject::System => {
                let system = System {
                    name: "System".to_string(), // Placeholder, could be anything
                };
                Subject::System(system)
            }
        };

        Self {
            id: audit_log.id.0.into(),
            subject,
            object: audit_log.object.as_ref().into(),
            action: audit_log.action.as_ref().into(),
            authorized: audit_log.authorized,
            created_at: audit_log.created_at.into(),
        }
    }
}
