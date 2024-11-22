#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use lava_app::app::LavaApp;

pub async fn run(app: LavaApp) -> anyhow::Result<()> {
    Ok(())
}
