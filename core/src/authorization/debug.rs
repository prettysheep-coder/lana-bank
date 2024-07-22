use super::{config::CasbinConfig, Authorization, AuthorizationError, LoanAction};

use crate::{
    authorization::{Action, Object},
    primitives::{Group, Subject},
};

pub async fn seed_permissions(config: &CasbinConfig) -> Result<(), AuthorizationError> {
    let mut auth = Authorization::init(config).await?;

    let _ = auth
        .add_permission(
            &Subject("admin".to_string()),
            Object::Loan,
            Action::Loan(LoanAction::Read),
        )
        .await;

    let _ = auth
        .add_permission(
            &Subject("admin".to_string()),
            Object::Loan,
            Action::Loan(LoanAction::List),
        )
        .await;

    let _ = auth
        .add_permission(
            &Subject("admin".to_string()),
            Object::Loan,
            Action::Loan(LoanAction::Create),
        )
        .await;

    let _ = auth
        .add_permission(
            &Subject("admin".to_string()),
            Object::Loan,
            Action::Loan(LoanAction::Approve),
        )
        .await;

    let _ = auth
        .add_grouping(&Subject("alice".to_string()), &Group("admin".to_string()))
        .await;

    Ok(())
}
