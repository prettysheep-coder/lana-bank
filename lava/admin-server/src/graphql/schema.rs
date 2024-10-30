use async_graphql::{types::connection::*, Context, Object};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "world".to_string()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello(&self) -> String {
        "world".to_string()
    }
}
