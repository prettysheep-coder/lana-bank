use sqlx::PgPool;

use es_entity::*;

use crate::{data_export::Export, primitives::ReportId};

use super::{entity::*, error::*};

const BQ_TABLE_NAME: &str = "report_events";

#[derive(EsRepo, Clone)]
#[es_repo(entity = "Report", err = "ReportError", post_persist_hook = "export")]
pub struct ReportRepo {
    pool: PgPool,
    export: Export,
}

impl ReportRepo {
    pub(super) fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }

    // pub(super) async fn create_in_tx(
    //     &self,
    //     db: &mut Transaction<'_, sqlx::Postgres>,
    //     new_report: NewReport,
    // ) -> Result<Report, ReportError> {
    //     sqlx::query!(
    //         r#"INSERT INTO reports (id)
    //         VALUES ($1)"#,
    //         new_report.id as ReportId,
    //     )
    //     .execute(&mut **db)
    //     .await?;
    //     let mut events = new_report.initial_events();
    //     events.persist(db).await?;
    //     Ok(Report::try_from(events)?)
    // }

    // pub async fn find_by_id(&self, id: ReportId) -> Result<Report, ReportError> {
    //     let rows = sqlx::query_as!(
    //         GenericEvent,
    //         r#"SELECT a.id, e.sequence, e.event,
    //             a.created_at AS entity_created_at, e.recorded_at AS event_recorded_at
    //         FROM reports a
    //         JOIN report_events e
    //         ON a.id = e.id
    //         WHERE a.id = $1"#,
    //         id as ReportId
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;
    //     match EntityEvents::load_first(rows) {
    //         Ok(user) => Ok(user),
    //         Err(EntityError::NoEntityEventsPresent) => Err(ReportError::CouldNotFindById(id)),
    //         Err(e) => Err(e.into()),
    //     }
    // }

    // pub async fn list(&self) -> Result<Vec<Report>, ReportError> {
    //     let rows = sqlx::query_as!(
    //         GenericEvent,
    //         r#"SELECT a.id, e.sequence, e.event,
    //             a.created_at AS entity_created_at, e.recorded_at AS event_recorded_at
    //         FROM reports a
    //         JOIN report_events e
    //         ON a.id = e.id
    //         ORDER BY a.created_at DESC, a.id, e.sequence"#,
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;
    //     let n = rows.len();
    //     let res = EntityEvents::load_n::<Report>(rows, n)?;
    //     Ok(res.0)
    // }

    // pub async fn update_in_tx(
    //     &self,
    //     db: &mut Transaction<'_, sqlx::Postgres>,
    //     report: &mut Report,
    // ) -> Result<(), ReportError> {
    //     report.events.persist(db).await?;
    //     Ok(())
    // }

    async fn export(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        events: impl Iterator<Item = &PersistedEvent<ReportEvent>>,
    ) -> Result<(), ReportError> {
        self.export
            .es_entity_export(db, BQ_TABLE_NAME, events)
            .await?;
        Ok(())
    }
}
