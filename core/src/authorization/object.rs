use std::{fmt::Display, str::FromStr};

use crate::primitives::{CustomerId, LoanId};

use super::error::AuthorizationError;

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum Object {
    Applicant,
    Loan(LoanRef),
    Term,
    User,
    Customer(CustomerRef),
    Deposit,
    Withdraw,
    Audit,
    Ledger,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Object::*;
        match self {
            Applicant => write!(f, "applicant"),
            Loan(loan_ref) => write!(f, "loan:{}", loan_ref),
            Term => write!(f, "term"),
            User => write!(f, "user"),
            Customer(customer_ref) => write!(f, "customer:{}", customer_ref),
            Deposit => write!(f, "deposit"),
            Withdraw => write!(f, "withdraw"),
            Audit => write!(f, "audit"),
            Ledger => write!(f, "ledger"),
        }
    }
}

impl FromStr for Object {
    type Err = AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "applicant" => Ok(Object::Applicant),
            s if s.starts_with("loan:") => {
                let loan_ref = s.trim_start_matches("loan:").parse().map_err(|_| {
                    AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    }
                })?;
                Ok(Object::Loan(loan_ref))
            }
            "term" => Ok(Object::Term),
            "user" => Ok(Object::User),
            s if s.starts_with("customer:") => {
                let customer_ref = s.trim_start_matches("customer:").parse().map_err(|_| {
                    AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    }
                })?;
                Ok(Object::Customer(customer_ref))
            }
            "deposit" => Ok(Object::Deposit),
            "withdraw" => Ok(Object::Withdraw),
            "audit" => Ok(Object::Audit),
            "ledger" => Ok(Object::Ledger),
            _ => Err(AuthorizationError::ObjectParseError {
                value: format!("Invalid Object: {}", s),
            }),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Ref<T> {
    All,
    ById(T),
}

impl<T> FromStr for Ref<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Ref::All),
            _ => {
                let id = T::from_str(s).map_err(|e| format!("Invalid ID: {}", e))?;
                Ok(Ref::ById(id))
            }
        }
    }
}

impl<T> Display for Ref<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ref::All => write!(f, "*"),
            Ref::ById(id) => write!(f, "{}", id),
        }
    }
}

pub type LoanRef = Ref<LoanId>;
pub type CustomerRef = Ref<CustomerId>;
