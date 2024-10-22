use sqlx::PgPool;

use es_entity::*;

use crate::{
    data_export::Export,
    primitives::{ApprovalProcessType, CommitteeId},
};

use super::{entity::*, error::*};

const BQ_TABLE_NAME: &str = "committee_events";

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "Committee",
    err = "CommitteeError",
    columns(approval_process_type = "ApprovalProcessType"),
    post_persist_hook = "export"
)]
pub struct CommitteeRepo {
    pool: PgPool,
    export: Export,
}

impl CommitteeRepo {
    pub fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }

    async fn export(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        events: impl Iterator<Item = &PersistedEvent<CommitteeEvent>>,
    ) -> Result<(), CommitteeError> {
        self.export
            .es_entity_export(db, BQ_TABLE_NAME, events)
            .await?;
        Ok(())
    }
}
