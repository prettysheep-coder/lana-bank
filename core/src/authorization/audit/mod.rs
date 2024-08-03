use chrono::{DateTime, Utc};

mod error;
use error::AuditError;

use uuid::Uuid;

use crate::primitives::Subject;

use super::{Action, Object};

pub struct NewAuditEvent<'a> {
    pub sub: &'a Subject,
    pub object: &'a Object,
    pub action: &'a Action,
    pub authorized: bool,
}

pub struct AuditEvent<'a> {
    pub id: Uuid,
    pub sub: &'a Subject,
    pub object: &'a Object,
    pub action: &'a Action,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

impl<'a> NewAuditEvent<'a> {
    pub fn into_audit_event(self) -> AuditEvent<'a> {
        AuditEvent {
            id: Uuid::new_v4(),
            sub: self.sub,
            object: self.object,
            action: self.action,
            authorized: self.authorized,
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone)]
pub struct Audit {
    pool: sqlx::PgPool,
}

impl Audit {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn log<'a>(&self, event: NewAuditEvent<'a>) -> Result<(), AuditError> {
        let event = event.into_audit_event();

        sqlx::query!(
            r#"
                INSERT INTO audit_events (id, subject, object, action, authorized, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            event.id,
            event.sub.as_ref(),
            event.object.as_ref(),
            event.action.as_ref(),
            event.authorized,
            event.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
