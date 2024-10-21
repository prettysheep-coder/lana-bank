use sqlx::PgPool;

use es_entity::*;

use crate::{
    data_export::Export,
    primitives::{CustomerId, DepositId},
};

use super::{entity::*, error::*, DepositCursor};

const BQ_TABLE_NAME: &str = "deposit_events";

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "Deposit",
    err = "DepositError",
    columns(
        customer_id = "CustomerId",
        reference(ty = "String", accessor(new = "reference()"))
    ),
    post_persist_hook = "export"
)]
pub struct DepositRepo {
    pool: PgPool,
    export: Export,
}

impl DepositRepo {
    pub(super) fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }

    pub async fn list_for_customer(
        &self,
        customer_id: CustomerId,
    ) -> Result<Vec<Deposit>, DepositError> {
        let (deposits, _) = es_entity::es_query!(
            &self.pool,
            "SELECT id FROM deposits WHERE customer_id = $1",
            customer_id as CustomerId,
        )
        .fetch_n(usize::MAX)
        .await?;

        Ok(deposits)
    }

    pub async fn list(
        &self,
        query: crate::query::PaginatedQueryArgs<DepositCursor>,
    ) -> Result<crate::query::PaginatedQueryRet<Deposit, DepositCursor>, DepositError> {
        let rows = sqlx::query_as!(
            GenericEvent,
            r#"
        WITH deposits AS (
            SELECT id, created_at
            FROM deposits
            WHERE created_at < $1 OR $1 IS NULL
            ORDER BY created_at DESC
            LIMIT $2
        )
        SELECT d.id as entity_id, e.sequence, e.event, e.recorded_at
        FROM deposits d
        JOIN deposit_events e ON d.id = e.id
        ORDER BY d.created_at DESC, d.id, e.sequence"#,
            query.after.map(|c| c.deposit_created_at),
            query.first as i64 + 1
        )
        .fetch_all(&self.pool)
        .await?;

        let (entities, has_next_page) = EntityEvents::load_n::<Deposit>(rows, query.first)?;

        let mut end_cursor = None;
        if let Some(last) = entities.last() {
            end_cursor = Some(DepositCursor {
                deposit_created_at: last.created_at(),
            });
        }

        Ok(crate::query::PaginatedQueryRet {
            entities,
            has_next_page,
            end_cursor,
        })
    }

    async fn export(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        events: impl Iterator<Item = &PersistedEvent<DepositEvent>>,
    ) -> Result<(), DepositError> {
        self.export
            .es_entity_export(db, BQ_TABLE_NAME, events)
            .await?;
        Ok(())
    }
}
