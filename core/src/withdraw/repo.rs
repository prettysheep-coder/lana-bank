use sqlx::PgPool;

use es_entity::*;

use crate::{
    data_export::Export,
    primitives::{CustomerId, WithdrawId},
};

use super::{entity::*, error::*};

const BQ_TABLE_NAME: &str = "withdraw_events";

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "Withdraw",
    err = "WithdrawError",
    columns(
        customer_id = "CustomerId",
        reference(ty = "String", accessor(new = "reference()")),
    ),
    post_persist_hook = "export"
)]
pub struct WithdrawRepo {
    pool: PgPool,
    export: Export,
}

impl WithdrawRepo {
    pub(super) fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }

    pub async fn list_for_customer(
        &self,
        customer_id: CustomerId,
    ) -> Result<Vec<Withdraw>, WithdrawError> {
        let rows = sqlx::query_as!(
            GenericEvent,
            r#"SELECT w.id AS entity_id, e.sequence, e.event, e.recorded_at 
               FROM withdraws w
               JOIN withdraw_events e ON w.id = e.id
               WHERE w.customer_id = $1
               ORDER BY w.id, e.sequence"#,
            customer_id as CustomerId,
        )
        .fetch_all(&self.pool)
        .await?;

        let n = rows.len();
        let deposits = EntityEvents::load_n(rows, n)?;
        Ok(deposits.0)
    }

    async fn export(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        events: impl Iterator<Item = &PersistedEvent<WithdrawEvent>>,
    ) -> Result<(), WithdrawError> {
        self.export
            .es_entity_export(db, BQ_TABLE_NAME, events)
            .await?;
        Ok(())
    }
}
