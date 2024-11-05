use thiserror::Error;

#[derive(Error, Debug)]
pub enum DashboardError {
    #[error("DashboardError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("DashboardError - JobError: {0}")]
    Job(#[from] ::job::error::JobError),
}
