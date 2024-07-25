mod entity;
pub mod error;
mod repo;

pub use entity::*;
use error::UserError;
pub use repo::UserRepo;

#[derive(Clone)]
pub struct Users {
    pool: sqlx::PgPool,
    repo: UserRepo,
}

impl Users {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        let repo = UserRepo::new(pool);
        Self {
            pool: pool.clone(),
            repo,
        }
    }

    pub fn repo(&self) -> &UserRepo {
        &self.repo
    }

    pub async fn create_user(&self, email: impl Into<String>) -> Result<User, UserError> {
        let new_user = NewUser::builder()
            .email(email)
            .build()
            .expect("Could not build customer");
        let mut db = self.pool.begin().await?;
        let user = self.repo.create_in_tx(&mut db, new_user).await?;
        db.commit().await?;
        Ok(user)
    }

    pub async fn find_by_email(&self, email: impl Into<String>) -> Result<Option<User>, UserError> {
        match self.repo.find_by_email(email).await {
            Ok(user) => Ok(Some(user)),
            Err(UserError::CouldNotFindByEmail(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
