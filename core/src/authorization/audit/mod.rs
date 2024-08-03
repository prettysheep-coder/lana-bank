use chrono::{DateTime, Utc};

// use serde::Serialize;
use uuid::Uuid;

pub struct NewAuditEvent<'a> {
    pub sub: &'a Uuid,
    pub object: &'a str,
    pub action: &'a str,
    pub authorized: bool,
}

pub struct AuditEvent<'a> {
    pub id: Uuid,
    pub sub: &'a Uuid,
    pub object: &'a str,
    pub action: &'a str,
    pub authorized: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Audit {
    pool: sqlx::PgPool,
}

impl Audit {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn log<'a>(&self, event: NewAuditEvent<'a>) {
        let event = AuditEvent {
            id: Uuid::new_v4(),
            sub: event.sub,
            object: event.object,
            action: event.action,
            authorized: event.authorized,
            created_at: Utc::now(),
        };

        println!(
            "Subject '{}' {} {} {}",
            event.sub,
            event.object,
            event.action,
            if event.authorized {
                "authorized"
            } else {
                "unauthorized"
            }
        );

        let res = sqlx::query!(
            r#"
                INSERT INTO audit_events (id, subject, object, action, authorized, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            event.id,
            event.sub,
            event.object,
            event.action,
            event.authorized,
            event.created_at
        )
        .execute(&self.pool)
        .await;

        println!("{:?}", res);
        // .await?;

        // self.log_to_db(event);
        // ...
    }
}
