use async_graphql::*;

#[derive(SimpleObject)]
#[graphql(name = "Subject", complex)]
pub struct AuthenticatedSubject {
    name: String,
}

#[ComplexObject]
impl AuthenticatedSubject {
    async fn age(&self) -> u32 {
        42
    }
}
