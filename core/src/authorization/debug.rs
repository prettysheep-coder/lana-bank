use super::{Authorization, AuthorizationError, LoanAction};

use crate::{
    authorization::{Action, Object},
    primitives::{Group, Subject},
};

pub async fn seed_permissions(pool: &sqlx::PgPool) -> Result<(), AuthorizationError> {
    let mut auth = Authorization::init(pool).await?;
    let subject = Subject::from("admin".to_string());

    let _ = auth
        .add_permission(&subject, Object::Loan, Action::Loan(LoanAction::Read))
        .await;

    let _ = auth
        .add_permission(&subject, Object::Loan, Action::Loan(LoanAction::List))
        .await;

    let _ = auth
        .add_permission(&subject, Object::Loan, Action::Loan(LoanAction::Create))
        .await;

    let _ = auth
        .add_permission(&subject, Object::Loan, Action::Loan(LoanAction::Approve))
        .await;

    let group = Group::from("admin".to_string());
    let alice = Subject::from("alice".to_string());

    let _ = auth.add_grouping(&alice, &group).await;

    Ok(())
}
