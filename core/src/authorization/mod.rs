use casbin::{prelude::Enforcer, CoreApi};
use std::sync::Arc;

pub mod error;
use error::AuthorizationError;

use crate::primitives::Subject;

#[derive(Clone)]
pub struct Authorization {
    enforcer: Arc<Enforcer>,
}

impl Authorization {
    pub async fn init() -> Result<Self, AuthorizationError> {
        let model_path = "dev/rbac.conf";
        let policy_path = "dev/policy.csv";

        let mut enforcer = Enforcer::new(model_path, policy_path).await?;
        enforcer.enable_log(true);
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
