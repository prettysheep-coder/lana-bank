use sqlx::{PgPool, Postgres, Transaction};

pub struct DbOp<'t> {
    tx: Transaction<'t, Postgres>,
    now: chrono::DateTime<chrono::Utc>,
}

impl<'t> DbOp<'t> {
    pub async fn init(
        pool: &PgPool,
        now: impl Into<Option<chrono::DateTime<chrono::Utc>>>,
    ) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let now = if let Some(now) = now.into() {
            now
        } else {
            sqlx::query!("SELECT NOW()")
                .fetch_one(&mut *tx)
                .await?
                .now
                .expect("NOW() is not NULL")
        };
        Ok(Self { tx, now })
    }

    pub fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.now
    }

    pub fn tx(&mut self) -> &mut Transaction<'t, Postgres> {
        &mut self.tx
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await?;
        Ok(())
    }
}
