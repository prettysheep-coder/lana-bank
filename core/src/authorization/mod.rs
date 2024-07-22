use std::sync::Arc;
use tokio::sync::RwLock;

pub mod config;
pub mod debug;
pub mod error;

use error::AuthorizationError;

use crate::primitives::{Group, Subject};
use sqlx_adapter::{
    casbin::{prelude::Enforcer, CoreApi, MgmtApi},
    SqlxAdapter,
};

#[derive(Clone)]
pub struct Authorization {
    enforcer: Arc<RwLock<Enforcer>>,
}

impl Authorization {
    pub async fn init(pool: &sqlx::PgPool) -> Result<Self, AuthorizationError> {
        let model_path = "dev/rbac.conf";

        let adapter = SqlxAdapter::new_with_pool(pool.clone()).await?;

        let enforcer = Enforcer::new(model_path, adapter).await?;
        Ok(Authorization {
            enforcer: Arc::new(RwLock::new(enforcer)),
        })
    }

    pub async fn check_permission(
        &self,
        sub: &Subject,
        object: Object,
        action: Action,
    ) -> Result<bool, AuthorizationError> {
        let enforcer = self.enforcer.read().await;

        match enforcer.enforce((sub.as_ref(), object.as_ref(), action.as_ref())) {
            Ok(true) => Ok(true),
            Ok(false) => Err(AuthorizationError::NotAuthorized),
            Err(e) => Err(AuthorizationError::Casbin(e)),
        }
    }

    pub async fn add_permission(
        &mut self,
        sub: &Subject,
        object: Object,
        action: Action,
    ) -> Result<(), AuthorizationError> {
        let mut enforcer = self.enforcer.write().await;

        enforcer
            .add_policy(vec![
                sub.to_string(),
                object.to_string(),
                action.to_string(),
            ])
            .await?;
        Ok(())
    }

    pub async fn add_grouping(
        &mut self,
        sub: &Subject,
        group: &Group,
    ) -> Result<(), AuthorizationError> {
        let mut enforcer = self.enforcer.write().await;

        enforcer
            .add_grouping_policy(vec![sub.to_string(), group.to_string()])
            .await?;

        Ok(())
    }
}

pub enum Object {
    Applicant,
    Loan,
}

impl AsRef<str> for Object {
    fn as_ref(&self) -> &str {
        match self {
            Object::Applicant => "applicant",
            Object::Loan => "loan",
        }
    }
}

impl std::ops::Deref for Object {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Object::Applicant => "applicant",
            Object::Loan => "loan",
        }
    }
}

pub enum Action {
    Loan(LoanAction),
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Action::Loan(action) => action.as_ref(),
        }
    }
}

impl std::ops::Deref for Action {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Action::Loan(action) => action.as_ref(),
        }
    }
}

pub enum LoanAction {
    List,
    Read,
    Create,
    Approve,
}

impl AsRef<str> for LoanAction {
    fn as_ref(&self) -> &str {
        match self {
            LoanAction::Read => "loan-read",
            LoanAction::Create => "loan-create",
            LoanAction::List => "loan-list",
            LoanAction::Approve => "loan-approve",
        }
    }
}

impl std::ops::Deref for LoanAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            LoanAction::Read => "loan-read",
            LoanAction::Create => "loan-create",
            LoanAction::List => "loan-list",
            LoanAction::Approve => "loan-approve",
        }
    }
}
