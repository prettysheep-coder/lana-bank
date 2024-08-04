use chrono::{DateTime, Utc};

mod error;
use error::AuditError;

use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    authorization::{Action, Object},
    primitives::Subject,
};

pub struct NewAuditLog {
    pub subject: Subject,
    pub object: Object,
    pub action: Action,
    pub authorized: bool,
}

pub struct AuditLog {
    pub id: Uuid,
    pub subject: Subject,
    pub object: Object,
    pub action: Action,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct RawAuditLog {
    id: Uuid,
    subject: Uuid,
    object: String,
    action: String,
    authorized: bool,
    created_at: DateTime<Utc>,
}

impl NewAuditLog {
    pub fn into_audit_event(self) -> AuditLog {
        AuditLog {
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

    pub async fn log(&self, event: NewAuditLog) -> Result<(), AuditError> {
        let event = event.into_audit_event();

        sqlx::query!(
            r#"
                INSERT INTO audit_logs (id, subject, object, action, authorized, created_at)
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

    pub async fn list(&self, _sub: &Subject) -> Result<Vec<AuditLog>, AuditError> {
        let raw_events: Vec<RawAuditLog> = sqlx::query_as!(
            RawAuditLog,
            r#"
            SELECT id, subject, object, action, authorized, created_at
            FROM audit_logs
            WHERE authorized = $1
            "#,
            true
        )
        .fetch_all(&self.pool)
        .await?;

        let events: Vec<AuditLog> = raw_events
            .into_iter()
            .map(|raw_event| AuditLog {
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
