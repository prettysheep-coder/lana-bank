use chrono::{DateTime, Utc};

mod error;
use error::AuditError;

use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    authorization::{Action, Object},
    primitives::Subject,
};

pub struct NewAuditEvent {
    pub subject: Subject,
    pub object: Object,
    pub action: Action,
    pub authorized: bool,
}

pub struct AuditEvent {
    pub id: Uuid,
    pub subject: Subject,
    pub object: Object,
    pub action: Action,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct RawAuditEvent {
    id: Uuid,
    subject: Uuid,
    object: String,
    action: String,
    authorized: bool,
    created_at: DateTime<Utc>,
}

impl NewAuditEvent {
    pub fn into_audit_event(self) -> AuditEvent {
        AuditEvent {
            id: Uuid::new_v4(),
            subject: self.subject,
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

    pub async fn log(&self, event: NewAuditEvent) -> Result<(), AuditError> {
        let event = event.into_audit_event();

        sqlx::query!(
            r#"
                INSERT INTO audit_events (id, subject, object, action, authorized, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            event.id,
            event.subject.as_ref(),
            event.object.as_ref(),
            event.action.as_ref(),
            event.authorized,
            event.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<AuditEvent>, AuditError> {
        let raw_events: Vec<RawAuditEvent> = sqlx::query_as!(
            RawAuditEvent,
            r#"
            SELECT id, subject, object, action, authorized, created_at
            FROM audit_events
            WHERE authorized = $1
            "#,
            true
        )
        .fetch_all(&self.pool)
        .await?;

        let events: Vec<AuditEvent> = raw_events
            .into_iter()
            .map(|raw_event| AuditEvent {
                id: raw_event.id,
                subject: Subject::from(raw_event.subject),
                object: Object::from(raw_event.object),
                action: Action::from(raw_event.action),
                authorized: raw_event.authorized,
                created_at: raw_event.created_at,
            })
            .collect();

        Ok(events)
    }
}
