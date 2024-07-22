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
        println!(
            "Checking permission for: {} {} {}",
            sub.0,
            object.as_str(),
            action.as_str()
        );

        let enforcer = self.enforcer.read().await;

        match enforcer.enforce((sub.0.as_str(), object.as_str(), action.as_str())) {
            Ok(true) => Ok(true),
            Ok(false) => Err(AuthorizationError::NotAuthorizedError),
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
                sub.0.clone(),
                object.as_str().to_string(),
                action.as_str().to_string(),
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
            .add_grouping_policy(vec![sub.0.clone(), group.0.to_string()])
            .await?;

        Ok(())
    }
}

pub enum Object {
    Applicant,
    Loan,
}

impl Object {
    fn as_str(&self) -> &str {
        match self {
            Object::Applicant => "applicant",
            Object::Loan => "loan",
        }
    }
}

pub enum Action {
    Loan(LoanAction),
}

pub enum LoanAction {
    List,
    Read,
    Create,
    Approve,
}

impl Action {
    fn as_str(&self) -> &str {
        match self {
            Action::Loan(action) => action.as_str(),
        }
    }
}

impl LoanAction {
    fn as_str(&self) -> &str {
        match self {
            LoanAction::Read => "loan-read",
            LoanAction::Create => "loan-create",
            LoanAction::List => "loan-list",
            LoanAction::Approve => "loan-approve",
        }
    }
}
