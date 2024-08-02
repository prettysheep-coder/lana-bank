mod account;
mod account_set;
mod customer;
mod loan;
mod schema;
mod shareholder_equity;
mod terms;
mod user;

use async_graphql::*;

pub use schema::*;

use crate::{app::LavaApp, server::admin::kratos::KratosClient};

pub fn schema(
    app: Option<LavaApp>,
    kratos: Option<KratosClient>,
) -> Schema<Query, Mutation, EmptySubscription> {
    let mut schema_builder = Schema::build(Query, Mutation, EmptySubscription);

    if let Some(app) = app {
        schema_builder = schema_builder.data(app);
    }

    if let Some(kratos) = kratos {
        schema_builder = schema_builder.data(kratos);
    }

    schema_builder.finish()
}
