use dashboard::*;
use lava_events::{CreditEvent, LavaEvent};

pub async fn init_pool() -> anyhow::Result<sqlx::PgPool> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());
    let pg_con = format!("postgres://user:password@{pg_host}:5433/pg");
    let pool = sqlx::PgPool::connect(&pg_con).await?;
    Ok(pool)
}

type Outbox = outbox::Outbox<LavaEvent>;

#[tokio::test]
async fn count_facilities() -> anyhow::Result<()> {
    let pool = init_pool().await.unwrap();
    sqlx::query!("DELETE FROM dashboards")
        .execute(&pool)
        .await?;
    let outbox = Outbox::init(&pool).await.unwrap();
    let mut jobs = ::job::Jobs::new(&pool, Default::default());
    let dashboard = Dashboard::init(&pool, &jobs, &outbox).await?;
    jobs.start_poll().await?;

    let mut tx = pool.begin().await?;
    outbox
        .persist(
            &mut tx,
            CreditEvent::CreditFacilityCreated {
                created_at: chrono::Utc::now(),
            },
        )
        .await?;
    tx.commit().await?;

    for _ in 0..10 {
        let row = sqlx::query!("SELECT COUNT(*) FROM dashboards")
            .fetch_one(&pool)
            .await?;
        if row.count.unwrap_or_default() > 0 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    let values = dashboard
        .load_for_time_range(TimeRange::ThisQuarter)
        .await?;
    assert_eq!(values.pending_facilities, 1);
    Ok(())
}
