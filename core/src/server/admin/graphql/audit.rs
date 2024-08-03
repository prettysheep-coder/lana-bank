use async_graphql::{SimpleObject, ID};

use crate::server::shared_graphql::primitives::{Timestamp, UUID};

#[derive(SimpleObject)]
pub struct AuditLogs {
    id: ID,
    subject: UUID,
    object: String,
    action: String,
    authorized: bool,
    created_at: Timestamp,
}
