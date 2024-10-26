pub mod error;

use sqlx_adapter::{
    casbin::{
        prelude::{DefaultModel, Enforcer},
        CoreApi, MgmtApi,
    },
    SqlxAdapter,
};
use std::{fmt, marker::PhantomData, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tracing::instrument;

use lava_audit::{AuditInfo, AuditSvc};

use error::AuthorizationError;

const MODEL: &str = include_str!("./rbac.conf");

#[derive(Clone)]
pub struct Authorization<Audit, R>
where
    Audit: AuditSvc,
{
    enforcer: Arc<RwLock<Enforcer>>,
    audit: Audit,
    _phantom: PhantomData<R>,
}

impl<Audit, R> Authorization<Audit, R>
where
    Audit: AuditSvc,
    R: FromStr + fmt::Display + fmt::Debug + Clone,
    <R as FromStr>::Err: fmt::Debug,
{
    pub async fn init(pool: &sqlx::PgPool, audit: &Audit) -> Result<Self, AuthorizationError> {
        let model = DefaultModel::from_str(MODEL).await?;
        let adapter = SqlxAdapter::new_with_pool(pool.clone()).await?;

        let enforcer = Enforcer::new(model, adapter).await?;

        let auth = Self {
            enforcer: Arc::new(RwLock::new(enforcer)),
            audit: audit.clone(),
            _phantom: PhantomData,
        };
        Ok(auth)
    }

    pub async fn add_role_hierarchy(
        &self,
        parent_role: R,
        child_role: R,
    ) -> Result<(), AuthorizationError> {
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .add_grouping_policy(vec![child_role.to_string(), parent_role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    #[instrument(name = "lava.authz.enforce_permission", skip(self))]
    pub async fn enforce_permission(
        &self,
        sub: &Audit::Subject,
        object: impl Into<Audit::Object> + std::fmt::Debug,
        action: impl Into<Audit::Action> + std::fmt::Debug,
    ) -> Result<AuditInfo<Audit::Subject>, AuthorizationError> {
        let object = object.into();
        let action = action.into();

        let result = self.inspect_permission(sub, object, action).await;
        match result {
            Ok(()) => Ok(self.audit.record_entry(sub, object, action, true).await?),
            Err(AuthorizationError::NotAuthorized) => {
                self.audit.record_entry(sub, object, action, false).await?;
                Err(AuthorizationError::NotAuthorized)
            }
            Err(e) => Err(e),
        }
    }

    #[instrument(name = "lava.authz.inspect_permission", skip(self))]
    pub async fn inspect_permission(
        &self,
        sub: &Audit::Subject,
        object: impl Into<Audit::Object> + std::fmt::Debug,
        action: impl Into<Audit::Action> + std::fmt::Debug,
    ) -> Result<(), AuthorizationError> {
        let object = object.into();
        let action = action.into();

        let mut enforcer = self.enforcer.write().await;
        enforcer.load_policy().await?;

        match enforcer.enforce((sub.to_string(), object.to_string(), action.to_string())) {
            Ok(true) => Ok(()),
            Ok(false) => Err(AuthorizationError::NotAuthorized),
            Err(e) => Err(AuthorizationError::Casbin(e)),
        }
    }

    pub async fn evaluate_permission(
        &self,
        sub: &Audit::Subject,
        object: impl Into<Audit::Object> + std::fmt::Debug,
        action: impl Into<Audit::Action> + std::fmt::Debug,
        enforce: bool,
    ) -> Result<Option<AuditInfo<Audit::Subject>>, AuthorizationError> {
        let object = object.into();
        let action = action.into();

        if enforce {
            Ok(Some(self.enforce_permission(sub, object, action).await?))
        } else {
            self.inspect_permission(sub, object, action)
                .await
                .map(|_| None)
        }
    }

    pub async fn add_permission_to_role(
        &self,
        role: &R,
        object: impl Into<Audit::Object>,
        action: impl Into<Audit::Action>,
    ) -> Result<(), AuthorizationError> {
        let object = object.into();
        let action = action.into();

        let mut enforcer = self.enforcer.write().await;
        match enforcer
            .add_policy(vec![
                role.to_string(),
                object.to_string(),
                action.to_string(),
            ])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    pub async fn assign_role_to_subject(
        &self,
        sub: impl Into<Audit::Subject>,
        role: &R,
    ) -> Result<(), AuthorizationError> {
        let sub = sub.into();
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .add_grouping_policy(vec![sub.to_string(), role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    pub async fn revoke_role_from_subject(
        &self,
        sub: impl Into<Audit::Subject>,
        role: &R,
    ) -> Result<(), AuthorizationError> {
        let sub = sub.into();
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .remove_grouping_policy(vec![sub.to_string(), role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthorizationError::from(e)),
        }
    }

    pub async fn roles_for_subject(
        &self,
        sub: impl Into<Audit::Subject>,
    ) -> Result<Vec<R>, AuthorizationError> {
        let sub = sub.into();
        let sub_uuid = sub.to_string();
        let enforcer = self.enforcer.read().await;

        let roles = enforcer
            .get_grouping_policy()
            .into_iter()
            .filter(|r| r[0] == sub_uuid)
            .map(|r| r[1].parse().expect("Could not parse role"))
            .collect();

        Ok(roles)
    }

    pub async fn check_all_permissions(
        &self,
        sub: &Audit::Subject,
        object: Audit::Object,
        actions: &[Audit::Action],
    ) -> Result<bool, AuthorizationError> {
        for action in actions {
            match self.enforce_permission(sub, object, *action).await {
                Ok(_) => continue,
                Err(AuthorizationError::NotAuthorized) => return Ok(false),
                Err(e) => return Err(e),
            }
        }
        Ok(true)
    }
}
