use std::env::{self, VarError};

pub struct Config {
    pub key: String,
    pub secret: String,
}

impl Config {
    pub fn build() -> Result<Config, VarError> {
        let key = env::var("BFX_KEY")?;
        let secret = env::var("BFX_SECRET")?;

        Ok(Config { key, secret })
    }
}
