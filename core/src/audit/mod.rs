use chrono::{DateTime, Utc};
use std::str::FromStr;

pub mod error;
use error::AuditError;

mod cursor;
pub use cursor::AuditCursor;

use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    authorization::{Action, Object},
    primitives::{AuditEntryId, Subject},
};

pub struct AuditEntry {
    pub id: AuditEntryId,
    pub subject: Subject,
    pub object: Object,
    pub action: Action,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct RawAuditEntry {
    id: AuditEntryId,
    subject: Uuid,
    endpoint: String,
    object: String,
    action: String,
    authorized: bool,
    created_at: DateTime<Utc>,
}

pub enum Endpoint {
    Admin,
    Public,
    System,
}

impl Endpoint {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Admin => "admin",
            Self::Public => "public",
            Self::System => "system",
        }
    }
}

impl FromStr for Endpoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Endpoint::Admin),
            "public" => Ok(Endpoint::Public),
            "system" => Ok(Endpoint::System),
            _ => Err(()),
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

    pub async fn persist(
        &self,
        subject: &Subject,
        object: Object,
        action: Action,
        authorized: bool,
    ) -> Result<(), AuditError> {
        let nil_uuid = Uuid::nil(); // Create a binding for the nil UUID
        let (sub, endpoint) = match subject {
            Subject::Admin(sub) => (sub, Endpoint::Admin),
            Subject::Public(sub) => (sub, Endpoint::Public),
            Subject::System => (nil_uuid.as_ref(), Endpoint::System), // Use the binding
        };

        sqlx::query!(
            r#"
                INSERT INTO audit_entries (subject, endpoint, object, action, authorized)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            sub,
            endpoint.as_str(),
            object.as_ref(),
            action.as_ref(),
            authorized,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list(
        &self,
        query: crate::query::PaginatedQueryArgs<AuditCursor>,
    ) -> Result<crate::query::PaginatedQueryRet<AuditEntry, AuditCursor>, AuditError> {
        // Extract the after_id and limit from the query
        let after_id: Option<i64> = query.after.map(|cursor| cursor.id);

        let limit = i64::try_from(query.first)?;

        // Fetch the raw events with pagination
        let raw_events: Vec<RawAuditEntry> = sqlx::query_as!(
            RawAuditEntry,
            r#"
            SELECT id, subject, endpoint, object, action, authorized, created_at
            FROM audit_entries
            WHERE ($1::BIGINT IS NULL OR id < $1::BIGINT)
            ORDER BY id DESC
            LIMIT $2
            "#,
            after_id,
            limit + 1,
        )
        .fetch_all(&self.pool)
        .await?;

        // Determine if there is a next page
        let has_next_page = raw_events.len() as i64 > limit;

        // If we fetched one extra, remove it from the results
        let events = if has_next_page {
            raw_events
                .into_iter()
                .take(limit.try_into().expect("can't convert to usize"))
                .collect()
        } else {
            raw_events
        };

        // Create the next cursor if there is a next page
        let end_cursor = if has_next_page {
            events.last().map(|event| AuditCursor { id: event.id.0 })
        } else {
            None
        };

        // Map raw events to the desired return type
        let audit_entries: Vec<AuditEntry> = events
            .into_iter()
            .map(|raw_event| {
                let endpoint =
                    Endpoint::from_str(&raw_event.endpoint).expect("Unexpected endpoint value");

                let subject = match endpoint {
                    Endpoint::Admin => Subject::Admin(raw_event.subject),
                    Endpoint::Public => Subject::Public(raw_event.subject),
                    Endpoint::System => Subject::System,
                };

                AuditEntry {
                    id: raw_event.id,
                    subject,
                    object: raw_event.object.parse().expect("Could not parse object"),
                    action: raw_event.action.parse().expect("Could not parse action"),
                    authorized: raw_event.authorized,
                    created_at: raw_event.created_at,
                }
            })
            .collect();

        // Return the paginated result
        Ok(crate::query::PaginatedQueryRet {
            entities: audit_entries,
            has_next_page,
            end_cursor,
        })
    }
}
