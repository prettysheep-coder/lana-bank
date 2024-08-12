use async_graphql::{SimpleObject, ID};
use uuid::Uuid;

use crate::server::shared_graphql::primitives::{Timestamp, UUID};

#[derive(SimpleObject)]
pub struct AuditEntry {
    id: ID,
    subject: UUID,
    // TODO proper enum
    subject_type: String,
    object: String,
    action: String,
    authorized: bool,
    created_at: Timestamp,
}

const NIL_UUID: uuid::Uuid = Uuid::nil();

impl From<crate::audit::AuditEntry> for AuditEntry {
    fn from(audit_log: crate::audit::AuditEntry) -> Self {
        let (subject, subject_type): (Uuid, String) = match audit_log.subject {
            crate::primitives::Subject::Admin(id) => (Uuid::from(id), "Admin".to_owned()),
            crate::primitives::Subject::Public(id) => (Uuid::from(id), "User".to_owned()),
            crate::primitives::Subject::System => (NIL_UUID, "System".to_owned()),
        };

        Self {
            id: audit_log.id.0.into(),
            subject: subject.into(),
            subject_type,
            object: audit_log.object.as_ref().into(),
            action: audit_log.action.as_ref().into(),
            authorized: audit_log.authorized,
            created_at: audit_log.created_at.into(),
        }
    }
}
