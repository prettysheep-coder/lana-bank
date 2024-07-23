use super::{Authorization, AuthorizationError, LoanAction};

use crate::{
    authorization::{Action, Object},
    primitives::{Role, Subject},
};

pub async fn seed_permissions(pool: &sqlx::PgPool) -> Result<(), AuthorizationError> {
    let mut auth = Authorization::init(pool).await?;

    let role = Role::SuperUser;

    let _ = auth
        .add_permission_to_role(&role, Object::Loan, Action::Loan(LoanAction::Read))
        .await;

    let _ = auth
        .add_permission_to_role(&role, Object::Loan, Action::Loan(LoanAction::List))
        .await;

    let _ = auth
        .add_permission_to_role(&role, Object::Loan, Action::Loan(LoanAction::Create))
        .await;

    let _ = auth
        .add_permission_to_role(&role, Object::Loan, Action::Loan(LoanAction::Approve))
        .await;

    let admin = Subject::from("admin");

    let _ = auth.assign_grouping_to_subject(&admin, &role).await;

    Ok(())
}
