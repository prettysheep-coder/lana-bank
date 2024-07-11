use std::sync::Arc;

pub mod config;
pub mod error;

use config::CasbinConfig;

use error::AuthorizationError;

use crate::primitives::Subject;
use sqlx_adapter::{
    casbin::{prelude::Enforcer, CoreApi, MgmtApi},
    SqlxAdapter,
};

pub struct Authorization {
    enforcer: Arc<Enforcer>,
}

const POOL_SIZE: u32 = 8;

impl Authorization {
    pub async fn init(config: &CasbinConfig) -> Result<Self, AuthorizationError> {
        let model_path = "dev/rbac.conf";

        let adapter = SqlxAdapter::new(config.db_con.as_str(), POOL_SIZE).await?;

        let enforcer = Enforcer::new(model_path, adapter).await?;
        Ok(Authorization {
            enforcer: Arc::new(enforcer),
        })
    }

    pub async fn check_permission(
        &self,
        sub: &Subject,
        object: Object,
        action: Action,
    ) -> Result<bool, AuthorizationError> {
        match self
            .enforcer
            .enforce((sub.0.as_str(), object.as_str(), action.as_str()))
        {
            Ok(true) => Ok(true),
            Ok(false) => Err(AuthorizationError::NotAuthorizedError),
            Err(e) => Err(AuthorizationError::Casbin(e)),
        }
    }

    // pub async fn add_permission(
    //     &mut self,
    //     sub: &Subject,
    //     object: Object,
    //     action: Action,
    // ) -> Result<(), AuthorizationError> {
    //     self.enforcer
    //         .add_policy(vec![
    //             sub.0.clone(),
    //             object.as_str().to_string(),
    //             action.as_str().to_string(),
    //         ])
    //         .await?;
    //     Ok(())
    // }
}

// object could be a trait on a Loan entity.

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
            LoanAction::Create => "loan-write",
            LoanAction::List => "loan-list",
        }
    }
}
