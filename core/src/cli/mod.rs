pub mod config;
mod db;

use anyhow::Context;
use clap::Parser;
use std::{fs, path::PathBuf};

use self::config::{Config, EnvOverride};

use std::sync::Arc;

use super::server::jwks::RemoteJwksDecoder;

#[derive(Parser)]
#[clap(long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        env = "LAVA_CONFIG",
        default_value = "lava.yml",
        value_name = "FILE"
    )]
    config: PathBuf,
    #[clap(
        long,
        env = "LAVA_HOME",
        default_value = ".lava",
        value_name = "DIRECTORY"
    )]
    lava_home: String,
    #[clap(env = "PG_CON")]
    pg_con: String,
    #[clap(env = "BITFINEX_KEY", default_value = "")]
    bfx_key: String,
    #[clap(env = "BITFINEX_SECRET", default_value = "")]
    bfx_secret: String,
    #[clap(env = "SUMSUB_KEY", default_value = "")]
    sumsub_key: String,
    #[clap(env = "SUMSUB_SECRET", default_value = "")]
    sumsub_secret: String,
    // TODO: callback secret
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::from_path(
        cli.config,
        EnvOverride {
            db_con: cli.pg_con,
            bfx_key: cli.bfx_key,
            bfx_secret: cli.bfx_secret,
            sumsub_key: cli.sumsub_key,
            sumsub_secret: cli.sumsub_secret,
        },
    )?;

    run_cmd(&cli.lava_home, config).await?;

    Ok(())
}

async fn run_cmd(lava_home: &str, config: Config) -> anyhow::Result<()> {
    lava_tracing::init_tracer(config.tracing)?;
    store_server_pid(lava_home, std::process::id())?;

    let (send, mut receive) = tokio::sync::mpsc::channel(1);
    let mut handles = Vec::new();
    let pool = db::init_pool(&config.db).await?;
    let app = crate::app::LavaApp::run(pool.clone(), config.app).await?;
    let admin_app = app.clone();

    let jwks_decoder = Arc::new(RemoteJwksDecoder::new(
        // TODO: move config to a jwks module so that it's not depend on public_server
        // as now it's appplicable also to admin API
        config.public_server.jwks_url.clone(),
    ));
    let decoder = jwks_decoder.clone();
    tokio::spawn(async move {
        decoder.refresh_keys_periodically().await;
    });

    let admin_send = send.clone();
    let jwks_decoder_clone = jwks_decoder.clone();

    handles.push(tokio::spawn(async move {
        let _ = admin_send.try_send(
            crate::server::admin::run(config.admin_server, admin_app, jwks_decoder_clone)
                .await
                .context("Admin server error"),
        );
    }));

    let api_send = send.clone();
    handles.push(tokio::spawn(async move {
        let _ = api_send.try_send(
            crate::server::public::run(config.public_server, app, jwks_decoder)
                .await
                .context("Public server error"),
        );
    }));

    let reason = receive.recv().await.expect("Didn't receive msg");
    for handle in handles {
        handle.abort();
    }

    reason
}

pub fn store_server_pid(lava_home: &str, pid: u32) -> anyhow::Result<()> {
    create_lava_dir(lava_home)?;
    let _ = fs::remove_file(format!("{lava_home}/server-pid"));
    fs::write(format!("{lava_home}/server-pid"), pid.to_string()).context("Writing PID file")?;
    Ok(())
}

fn create_lava_dir(lava_home: &str) -> anyhow::Result<()> {
    let _ = fs::create_dir(lava_home);
    Ok(())
}
