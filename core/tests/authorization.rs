mod helpers;

use lava_core::{
    audit::*,
    authorization::{error::AuthorizationError, *},
    primitives::*,
};

#[tokio::test]
async fn roles_permission() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let audit = Audit::new(&pool);
    let authz = Authorization::init(&pool, audit).await?;

    let superuser_id = uuid::Uuid::new_v4();
    let superuser_subject = Subject::from(superuser_id);
    authz
        .assign_role_to_subject(&superuser_subject, &Role::Superuser)
        .await?;

    let admin_id = uuid::Uuid::new_v4();
    let admin_subject = Subject::from(admin_id);
    authz
        .assign_role_to_subject(&admin_subject, &Role::Admin)
        .await?;

    let bank_manager_id = uuid::Uuid::new_v4();
    let bank_manager_subject = Subject::from(bank_manager_id);
    authz
        .assign_role_to_subject(&bank_manager_subject, &Role::BankManager)
        .await?;

    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::Create),
        )
        .await
        .is_ok());

    assert!(authz
        .check_permission(
            &admin_subject,
            Object::User,
            Action::User(UserAction::Create)
        )
        .await
        .is_ok());

    assert!(matches!(
        authz
            .check_permission(
                &bank_manager_subject,
                Object::User,
                Action::User(UserAction::Create)
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    assert!(matches!(
        authz
            .check_permission(
                &superuser_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::Superuser))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::Admin))
        )
        .await
        .is_ok());

    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::BankManager))
        )
        .await
        .is_ok());

    assert!(matches!(
        authz
            .check_permission(
                &admin_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::Admin))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    assert!(authz
        .check_permission(
            &admin_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::BankManager))
        )
        .await
        .is_ok());

    assert!(matches!(
        authz
            .check_permission(
                &bank_manager_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::BankManager))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    Ok(())
}
