#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use std::{collections::HashMap, fmt, marker::PhantomData, str::FromStr};

use chrono::{DateTime, Utc};

mod cursor;
pub mod error;
mod primitives;

pub use cursor::AuditCursor;
use error::AuditError;
pub use primitives::*;

pub struct AuditEntry<S, O, A> {
    pub id: AuditEntryId,
    pub subject: S,
    pub object: O,
    pub action: A,
    pub authorized: bool,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Audit<S, O, A> {
    pool: sqlx::PgPool,
    _subject: PhantomData<S>,
    _object: PhantomData<O>,
    _action: PhantomData<A>,
}

impl<S, O, A> Audit<S, O, A>
where
    S: FromStr + fmt::Display + Clone,
    O: FromStr + fmt::Display + Copy,
    A: FromStr + fmt::Display + Copy,
    <S as FromStr>::Err: fmt::Debug,
    <O as FromStr>::Err: fmt::Debug,
    <A as FromStr>::Err: fmt::Debug,
{
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self {
            pool: pool.clone(),
            _subject: std::marker::PhantomData,
            _object: std::marker::PhantomData,
            _action: std::marker::PhantomData,
        }
    }

    pub async fn record_entry(
        &self,
        subject: &S,
        object: impl Into<O>,
        action: impl Into<A>,
        authorized: bool,
    ) -> Result<AuditInfo<S>, AuditError> {
        let object = object.into();
        let action = action.into();

        let record = sqlx::query!(
            r#"
                INSERT INTO audit_entries (subject, object, action, authorized)
                VALUES ($1, $2, $3, $4)
                RETURNING id, subject
                "#,
            subject.to_string(),
            object.to_string(),
            action.to_string(),
            authorized,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(AuditInfo::from((record.id, subject.clone())))
    }

    pub async fn list(
        &self,
        query: es_entity::PaginatedQueryArgs<AuditCursor>,
    ) -> Result<es_entity::PaginatedQueryRet<AuditEntry<S, O, A>, AuditCursor>, AuditError> {
        let after_id: Option<AuditEntryId> = query.after.map(|cursor| cursor.id);
        let limit = query.first;

        let rows = sqlx::query!(
            r#"
            SELECT id AS "id: AuditEntryId", subject, object, action, authorized, recorded_at
            FROM audit_entries
            WHERE COALESCE(id < $1, true)
            ORDER BY id DESC
            LIMIT $2
            "#,
            after_id as Option<AuditEntryId>,
            (limit + 1) as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        let has_next_page = rows.len() > limit;

        let entries: Vec<AuditEntry<_, _, _>> = rows
            .into_iter()
            .take(limit)
            .map(|raw_event| AuditEntry {
                id: raw_event.id,
                subject: raw_event.subject.parse().expect("Could not parse subject"),
                object: raw_event.object.parse().expect("Could not parse object"),
                action: raw_event.action.parse().expect("Could not parse action"),
                authorized: raw_event.authorized,
                recorded_at: raw_event.recorded_at,
            })
            .collect();

        let end_cursor = if has_next_page {
            entries.last().map(|event| AuditCursor { id: event.id })
        } else {
            None
        };

        Ok(es_entity::PaginatedQueryRet {
            entities: entries,
            has_next_page,
            end_cursor,
        })
    }

    pub async fn find_all<T: From<AuditEntry<S, O, A>>>(
        &self,
        ids: &[AuditEntryId],
    ) -> Result<HashMap<AuditEntryId, T>, AuditError> {
        let raw_entries = sqlx::query!(
            r#"
            SELECT id AS "id: AuditEntryId", subject, object, action, authorized, recorded_at
            FROM audit_entries
            WHERE id = ANY($1)
            "#,
            &ids as &[AuditEntryId],
        )
        .fetch_all(&self.pool)
        .await?;

        let audit_entries: HashMap<AuditEntryId, T> = raw_entries
            .into_iter()
            .map(|raw_entry| {
                let audit_entry = AuditEntry {
                    id: raw_entry.id,
                    subject: raw_entry.subject.parse().expect("Could not parse subject"),
                    object: raw_entry.object.parse().expect("Could not parse object"),
                    action: raw_entry.action.parse().expect("Could not parse action"),
                    authorized: raw_entry.authorized,
                    recorded_at: raw_entry.recorded_at,
                };
                (raw_entry.id, T::from(audit_entry))
            })
            .collect();

        Ok(audit_entries)
    }
}
