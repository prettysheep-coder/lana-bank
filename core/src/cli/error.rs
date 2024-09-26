use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("CliError - YAML Error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("CliError - Couldn't read config file: {0}")]
    ConfigReadError(String),
    #[error("CliError - ReportError: {0}")]
    ReportError(#[from] crate::report::error::ReportError),
}
