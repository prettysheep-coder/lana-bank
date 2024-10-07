use sqlx::{PgPool, Postgres, Transaction};

use super::{entity::*, error::*};
use crate::{entity::*, primitives::JobId};

#[derive(Debug, Clone)]
pub(super) struct JobRepo {
    pool: PgPool,
}

impl JobRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_in_tx(
        &self,
        db: &mut Transaction<'_, Postgres>,
        new_job: NewJob,
    ) -> Result<Job, JobError> {
        let id = new_job.id;
        sqlx::query!(
            r#"INSERT INTO jobs (id, name)
            VALUES ($1, $2)"#,
            id as JobId,
            new_job.name,
        )
        .execute(&mut **db)
        .await?;
        let mut events = new_job.initial_events();
        events.persist(db).await?;
        let job = Job::try_from(events)?;
        Ok(job)
    }

    pub async fn persist(
        &self,
        db: &mut Transaction<'_, Postgres>,
        mut job: Job,
    ) -> Result<(), JobError> {
        job.events.persist(db).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: JobId) -> Result<Job, JobError> {
        Ok(sqlx::query_as!(
            Job,
            r#"SELECT id, name, created_at, job_type, config
            FROM jobs
            WHERE id = $1"#,
            id as JobId
        )
        .fetch_one(&self.pool)
        .await?)
    }
}
