use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("AuthorizationError - CasbinError: {0}")]
    Casbin(#[from] casbin::error::Error),
    #[error("AuthorizationError - NotAuthorizedError")]
    NotAuthorizedError,
}
