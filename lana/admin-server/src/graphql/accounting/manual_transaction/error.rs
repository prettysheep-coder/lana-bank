use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManualTransactionInputError {
    #[error("ManualTransactionError - Currency {0} not supported")]
    CurrencyNotSupported(String),
    #[error("ManualTransactionError - {0} is neither a valid account ID nor code")]
    AccountIdOrCodeInvalid(String),
}
