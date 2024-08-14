mod helpers;

use serial_test::file_serial;

use lava_core::{
    audit::*,
    authorization::{error::AuthorizationError, *},
    primitives::*,
    user::{UserConfig, Users},
};
use uuid::Uuid;

fn random_email() -> String {
    format!(
        "superuser+{}@integrationtest.com",
        Uuid::new_v4().to_string()
    )
}

async fn create_user(authz: Authorization, users: Users) -> UserId {
    let _ = authz
        .add_policy_for_test(
            format!("system:{}", Uuid::nil()).as_str(),
            Object::User,
            Action::User(UserAction::Create),
        )
        .await;

    let subject = &Subject::System;
    let user = users.create_user(subject, random_email()).await;

    user.expect("impossible to unwrap user").id
}

#[tokio::test]
#[file_serial]
async fn superuser_permissions() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let audit = Audit::new(&pool);
    let authz = Authorization::init(&pool, audit).await?;
    let users = Users::init(
        &pool,
        &authz,
        UserConfig {
            superuser_email: None,
        },
    )
    .await?;

    let superuser_id = create_user(authz.clone(), users.clone()).await;
    let superuser_subject = Subject::from(superuser_id);
    authz
        .assign_role_to_subject(superuser_subject, &Role::Superuser)
        .await?;

    // Superuser can create users
    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::Create),
        )
        .await
        .is_ok());

    // Superuser can assign Admin role
    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::Admin))
        )
        .await
        .is_ok());

    // Superuser can assign Bank Manager role
    assert!(authz
        .check_permission(
            &superuser_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::BankManager))
        )
        .await
        .is_ok());

    // Superuser cannot assign Superuser role
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

    Ok(())
}

#[tokio::test]
#[file_serial]
async fn admin_permissions() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let audit = Audit::new(&pool);
    let authz = Authorization::init(&pool, audit).await?;
    let users = Users::init(
        &pool,
        &authz,
        UserConfig {
            superuser_email: None,
        },
    )
    .await?;

    let admin_id = create_user(authz.clone(), users.clone()).await;
    let admin_subject = Subject::from(admin_id);
    authz
        .assign_role_to_subject(admin_subject, &Role::Admin)
        .await?;

    // Admin can create users
    assert!(authz
        .check_permission(
            &admin_subject,
            Object::User,
            Action::User(UserAction::Create)
        )
        .await
        .is_ok());

    // Admin can assign Bank Manager role
    assert!(authz
        .check_permission(
            &admin_subject,
            Object::User,
            Action::User(UserAction::AssignRole(Role::BankManager))
        )
        .await
        .is_ok());

    // Admin cannot assign Admin role
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

    // Admin cannot assign Superuser role
    assert!(matches!(
        authz
            .check_permission(
                &admin_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::Superuser))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    Ok(())
}

#[tokio::test]
#[file_serial]
async fn bank_manager_permissions() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let audit = Audit::new(&pool);
    let authz = Authorization::init(&pool, audit).await?;

    let users = Users::init(
        &pool,
        &authz,
        UserConfig {
            superuser_email: None,
        },
    )
    .await?;

    let bank_manager_id = create_user(authz.clone(), users.clone()).await;
    let bank_manager_subject = Subject::from(bank_manager_id);
    authz
        .assign_role_to_subject(bank_manager_subject, &Role::BankManager)
        .await?;

    // Bank Manager cannot create users
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

    // Bank Manager cannot assign roles
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

    assert!(matches!(
        authz
            .check_permission(
                &bank_manager_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::Admin))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    assert!(matches!(
        authz
            .check_permission(
                &bank_manager_subject,
                Object::User,
                Action::User(UserAction::AssignRole(Role::Superuser))
            )
            .await,
        Err(AuthorizationError::NotAuthorized)
    ));

    Ok(())
}
