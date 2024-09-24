use lava_core::{applicant::*, primitives::CustomerId};
use sumsub_auth::{AccessTokenResponse, PermalinkResponse, SumsubClient};

use std::env;
use tokio;
use uuid::Uuid;

fn load_config_from_env() -> Option<SumsubConfig> {
    let sumsub_key = env::var("SUMSUB_KEY").ok()?;
    let sumsub_secret = env::var("SUMSUB_SECRET").ok()?;

    Some(SumsubConfig {
        sumsub_key,
        sumsub_secret,
    })
}

#[tokio::test]
async fn get_access_token() {
    let user_config = load_config_from_env();

    if user_config.is_none() {
        println!("not running the test");
        return;
    };

    let user_config = user_config.unwrap();
    let v = SumsubClient::new(&user_config);

    let random_id = Uuid::new_v4();
    let user_id = CustomerId::from(random_id);
    let level = "basic-kyc-level";

    let res = v.create_access_token(user_id.clone(), level).await;

    match res {
        Ok(AccessTokenResponse {
            token,
            user_id: returned_user_id,
        }) => {
            assert!(!token.is_empty(), "The returned token should not be empty");
            assert_eq!(
                user_id.to_string(),
                returned_user_id,
                "The returned user_id should match the input user_id"
            );
        }
        Err(e) => {
            panic!("Request failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn create_permalink() {
    let user_config = load_config_from_env();

    if user_config.is_none() {
        println!("not running the test");
        return;
    };

    let user_config = user_config.unwrap();
    let v = SumsubClient::new(&user_config);

    let random_id = Uuid::new_v4();
    let user_id = CustomerId::from(random_id);
    let level = "basic-kyc-level";

    let res = v.create_permalink(user_id, level).await;

    match res {
        Ok(PermalinkResponse { url }) => {
            assert!(!url.is_empty(), "The returned URL should not be empty");
            assert!(url.starts_with("http"), "The URL should start with 'http'");
        }
        Err(e) => {
            panic!("Request failed: {:?}", e);
        }
    }
}
