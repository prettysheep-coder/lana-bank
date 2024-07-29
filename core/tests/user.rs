mod helpers;
use rand::distributions::{Alphanumeric, DistString};

use lava_core::{app::*, primitives::*, user::*};

fn generate_random_email() -> String {
    let random_string: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    format!("{}@example.com", random_string.to_lowercase())
}

#[tokio::test]
async fn bank_manager_lifecycle() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let super_user_email = generate_random_email();
    let super_user_subject = Subject::from(super_user_email.clone());
    let user_config = UserConfig {
        super_user_email: Some(super_user_email),
    };
    let app_config = AppConfig {
        user: user_config,
        ..Default::default()
    };
    let app = LavaApp::run(pool, app_config).await?;

    let user_email = generate_random_email();
    let user = app
        .users()
        .create_user(&super_user_subject, user_email.clone())
        .await?;
    assert_eq!(user.email, user_email);

    let bank_manager = app
        .users()
        .assign_role_to_user(&super_user_subject, user.id, Role::BankManager)
        .await;

    assert!(bank_manager.is_ok());
    let bank_manager_id = bank_manager?.id;

    assert_eq!(
        app.users()
            .roles_for_user(&super_user_subject, bank_manager_id)
            .await?,
        vec![Role::BankManager]
    );

    app.users()
        .revoke_role_from_user(&super_user_subject, bank_manager_id, Role::BankManager)
        .await?;

    assert_eq!(
        app.users()
            .roles_for_user(&super_user_subject, bank_manager_id)
            .await?,
        vec![]
    );

    Ok(())
}
