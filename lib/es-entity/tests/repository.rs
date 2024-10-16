mod user_entity;

use es_entity::*;

use user_entity::*;

#[derive(EsRepo)]
#[es_repo(entity = "User", indexes(email = String))]
pub struct Users {}

pub async fn init_pool() -> anyhow::Result<sqlx::PgPool> {
    let pg_host = std::env::var("PG_HOST").unwrap_or("localhost".to_string());
    let pg_con = format!("postgres://user:password@{pg_host}:5433/pg");
    let pool = sqlx::PgPool::connect(&pg_con).await?;
    Ok(pool)
}

#[tokio::test]
async fn test_create() -> anyhow::Result<()> {
    let pool = init_pool().await?;

    let id = UserId::from(uuid::Uuid::new_v4());
    let repo = Users {};

    let mut db = pool.begin().await?;
    let entity = repo
        .create(
            &mut db,
            NewUser {
                id,
                email: "email@test.com".to_string(),
            },
        )
        .await?;
    assert!(entity.id == id);

    Ok(())
}

#[tokio::test]
async fn test_find() -> anyhow::Result<()> {
    let pool = init_pool().await?;

    let repo = Users {};

    let mut db = pool.begin().await?;
    let res = repo
        .find_by_email(&mut db, "email@test.com".to_string())
        .await;

    assert!(matches!(
        res,
        Err(EsRepoError::EntityError(EsEntityError::NotFound))
    ));

    Ok(())
}
