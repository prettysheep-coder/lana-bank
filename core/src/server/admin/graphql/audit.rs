use std::primitive;

use async_graphql::{dataloader::DataLoader, ComplexObject, Context, SimpleObject, Union, ID};

use crate::{
    customer::Customers,
    primitives,
    server::{
        admin::graphql::user::User,
        shared_graphql::{customer::Customer, primitives::Timestamp},
    },
    user::Users,
};

use crate::server::admin::graphql::LavaDataLoader;

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
#[graphql(complex)]
pub struct AuditEntry {
    id: ID,
    #[graphql(skip)]
    subject_str: String,
    object: String,
    action: String,
    authorized: bool,
    created_at: Timestamp,
}

#[ComplexObject]
impl AuditEntry {
    async fn subject(&self, ctx: &Context<'_>) -> async_graphql::Result<Subject> {
        let loader = ctx.data_unchecked::<DataLoader<LavaDataLoader>>();

        let subject: primitives::Subject =
            self.subject_str.parse().expect("decoding subject error");

        match subject {
            primitives::Subject::User(id) => {
                let user = loader.load_one(id).await?;
                match user {
                    None => return Err("User not found".into()),
                    Some(user) => {
                        let user = User::from(user);
                        Ok(Subject::User(user))
                    }
                }
            }

            primitives::Subject::Customer(id) => {
                unimplemented!("Customer parsing not implemented");

                // let customer = loader.load_one(id).await?;
                // match customer {
                //     None => return Err("Customer not found".into()),
                //     Some(customer) => {
                //         let customer = Customer::from(customer);
                //         Ok(Subject::Customer(customer))
                //     }
                // }
            }
            primitives::Subject::System => {
                let system = System {
                    name: "System".to_string(), // Placeholder, could be anything
                };
                Ok(Subject::System(system))
            }
        }
    }
}

impl From<crate::audit::AuditEntry> for AuditEntry {
    fn from(entry: crate::audit::AuditEntry) -> Self {
        Self {
            id: entry.id.0.into(),
            subject_str: entry.subject.to_string(),
            object: entry.object.as_ref().into(),
            action: entry.action.as_ref().into(),
            authorized: entry.authorized,
            created_at: entry.created_at.into(),
        }
    }
}
