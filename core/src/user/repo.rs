use sqlx::PgPool;

use super::{entity::*, error::*};
use crate::{entity::*, primitives::*};

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub(super) fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub(super) async fn create(&self, new_user: NewUser) -> Result<EntityUpdate<User>, UserError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"INSERT INTO users (id, bitfinex_username)
            VALUES ($1, $2)"#,
            new_user.id as UserId,
            new_user.bitfinex_username,
        )
        .execute(&mut *tx)
        .await?;
        let mut events = new_user.initial_events();
        let n_new_events = events.persist(&mut tx).await?;
        tx.commit().await?;
        let user = User::try_from(events)?;
        Ok(EntityUpdate {
            entity: user,
            n_new_events,
        })
    }

    pub async fn find_by_id(&self, user_id: UserId) -> Result<User, UserError> {
        let rows = sqlx::query_as!(
            GenericEvent,
            r#"SELECT a.id, e.sequence, e.event,
                a.created_at AS entity_created_at, e.recorded_at AS event_recorded_at
            FROM users a
            JOIN user_events e
            ON a.id = e.id
            WHERE a.id = $1"#,
            user_id as UserId
        )
        .fetch_all(&self.pool)
        .await?;
        match EntityEvents::load_first(rows) {
            Ok(user) => Ok(user),
            Err(EntityError::NoEntityEventsPresent) => Err(UserError::CouldNotFindById(user_id)),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn persist_in_tx(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        settings: &mut User,
    ) -> Result<(), UserError> {
        settings.events.persist(db).await?;
        Ok(())
    }

    pub async fn list(
        &self,
        first: usize,
        after: Option<UserId>,
    ) -> Result<(Vec<User>, bool), UserError> {
        let rows = sqlx::query_as!(
            GenericEvent,
            r#"WITH anchor AS (
                 SELECT created_at FROM users WHERE id = $1 LIMIT 1
               )
            SELECT a.id, e.sequence, e.event,
                      a.created_at AS entity_created_at, e.recorded_at AS event_recorded_at
            FROM users a
            JOIN user_events e ON a.id = e.id
            WHERE (
                    $1 IS NOT NULL AND a.created_at < (SELECT created_at FROM anchor)
                    OR $1 IS NULL)
            ORDER BY a.created_at DESC, a.id, e.sequence
            LIMIT $2"#,
            after as Option<UserId>,
            first as i64 + 1,
        )
        .fetch_all(&self.pool)
        .await?;
        let res = EntityEvents::load_n::<User>(rows, first)?;
        Ok(res)
    }
}
