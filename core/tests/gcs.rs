mod helpers;

use lava_core::{
    cli::config::Config,
    storage::{config::StorageConfig, Storage},
};

#[tokio::test]
async fn upload_doc() -> anyhow::Result<()> {
    let sa_creds_base64 = std::env::var("SA_CREDS_BASE64")?;

    let config_file =
        std::fs::read_to_string("../bats/lava.yml").expect("Couldn't read config file");

    let config: Config = serde_yaml::from_str(&config_file).expect("Couldn't parse config file");

    let mut service_account = config.app.service_account;
    service_account.set_sa_creds_base64(sa_creds_base64)?;
    std::env::set_var("SERVICE_ACCOUNT_JSON", service_account.get_json_creds()?);

    let mut storage = Storage::new(&config.app.storage);

    if let Ok(name_prefix) = std::env::var("DEV_ENV_NAME_PREFIX") {
        let docs_config = StorageConfig::new_dev_mode(name_prefix);
        storage = Storage::new(&docs_config);
    }

    let file = "test".as_bytes().to_vec();
    let filename = "test.txt";

    let _ = storage.upload(file, filename, "application/txt").await;

    let res = storage._list("".to_string()).await?;

    assert!(res.get(0) == Some(&filename.to_owned()));
    Ok(())
}
