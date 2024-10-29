#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod entity;
pub mod error;
mod primitives;
mod repo;

use audit::AuditSvc;
use authz::{Authorization, PermissionCheck};
use outbox::Outbox;

use entity::*;
use error::*;
pub use primitives::*;
use repo::*;

pub struct Users<Audit, E>
where
    Audit: AuditSvc,
    E: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static + Unpin,
{
    pool: sqlx::PgPool,
    authz: Authorization<Audit, Role>,
    outbox: Outbox<E>,
    repo: UserRepo,
}
impl<Audit, E> Users<Audit, E>
where
    Audit: AuditSvc,
    <Audit as AuditSvc>::Subject: From<UserId>,
    <Audit as AuditSvc>::Action: From<UserModuleAction>,
    <Audit as AuditSvc>::Object: From<UserObject>,
    E: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static + Unpin,
{
    pub async fn init(
        pool: &sqlx::PgPool,
        authz: &Authorization<Audit, Role>,
        super_user: Option<SuperuserInit>,
        outbox: &Outbox<E>,
    ) -> Result<Self, UserError> {
        let repo = UserRepo::new(pool);
        let users = Self {
            pool: pool.clone(),
            repo,
            authz: authz.clone(),
            outbox: outbox.clone(),
        };

        if let Some(SuperuserInit { email, role }) = super_user {
            users
                .create_and_assign_role_to_superuser(email, role)
                .await?;
        }

        Ok(users)
    }

    async fn create_and_assign_role_to_superuser(
        &self,
        email: String,
        role: Role,
    ) -> Result<(), UserError> {
        let mut db = self.pool.begin().await?;

        let audit_info = self
            .authz
            .audit()
            .record_system_entry_in_tx(
                &mut db,
                UserObject::User(UserAllOrOne::All),
                UserModuleAction::User(UserEntityAction::Create),
            )
            .await?;

        match self.repo.find_by_email_in_tx(&mut db, &email).await {
            Err(UserError::NotFound) => {
                let new_user = NewUser::builder()
                    .email(&email)
                    .audit_info(audit_info.clone())
                    .build()
                    .expect("Could not build user");
                let mut user = self.repo.create_in_tx(&mut db, new_user).await?;
                self.authz.assign_role_to_subject(user.id, &role).await?;
                user.assign_role(role, audit_info);
                self.repo.update_in_tx(&mut db, &mut user).await?;
                db.commit().await?;
            }
            Err(e) => return Err(e),
            Ok(mut user) => {
                self.authz.assign_role_to_subject(user.id, &role).await?;
                user.assign_role(role, audit_info);
                self.repo.update_in_tx(&mut db, &mut user).await?;
                db.commit().await?;
            }
        }
        Ok(())
    }
}
