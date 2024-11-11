use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::DatabaseOperation;

pub struct DbOp<'t> {
    tx: Transaction<'t, Postgres>,
    now: chrono::DateTime<chrono::Utc>,
}

impl<'t> DbOp<'t> {
    pub async fn init(
        pool: &PgPool,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, sqlx::Error> {
        let tx = pool.begin().await?;
        Ok(Self { tx, now })
    }
}

#[async_trait]
impl<'t> DatabaseOperation<'t> for DbOp<'t> {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.now
    }

    fn tx(&'t mut self) -> &mut Transaction<'t, Postgres> {
        &mut self.tx
    }

    async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await?;
        Ok(())
    }
}
