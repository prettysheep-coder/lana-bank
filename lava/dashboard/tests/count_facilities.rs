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
    let outbox = Outbox::init(&pool).await.unwrap();
    let dashboard = Dashboard::new(&outbox);

    let mut tx = pool.begin().await?;
    outbox
        .persist(&mut tx, CreditEvent::CreditFacilityCreated)
        .await?;
    tx.commit().await?;

    let dashboard = dashboard
        .load_for_time_range(TimeRange::LastQuarter)
        .await?;
    assert_eq!(dashboard.active_facilities, 1);

    Ok(())
}
