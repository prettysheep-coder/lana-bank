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
            Applicant => write!(f, "{}", ObjectDiscriminants::Applicant),
            Loan(loan_ref) => write!(f, "{}:{}", ObjectDiscriminants::Loan, loan_ref),
            Term => write!(f, "{}", ObjectDiscriminants::Term),
            User => write!(f, "{}", ObjectDiscriminants::User),
            Customer(customer_ref) => {
                write!(f, "{}:{}", ObjectDiscriminants::Customer, customer_ref)
            }
            Deposit => write!(f, "{}", ObjectDiscriminants::Deposit),
            Withdraw => write!(f, "{}", ObjectDiscriminants::Withdraw),
            Audit => write!(f, "{}", ObjectDiscriminants::Audit),
            Ledger => write!(f, "{}", ObjectDiscriminants::Ledger),
        }
    }
}

impl FromStr for Object {
    type Err = AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elems = s.split(':');
        let entity = elems.next().expect("missing first element");
        use ObjectDiscriminants::*;
        let res = match entity.parse()? {
            Applicant => Object::Applicant,
            Loan => {
                let loan_ref = elems
                    .next()
                    .ok_or(AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    })?
                    .parse()
                    .map_err(|_| AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    })?;
                Object::Loan(loan_ref)
            }
            Term => Object::Term,
            User => Object::User,
            Customer => {
                let customer_ref = elems
                    .next()
                    .ok_or(AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    })?
                    .parse()
                    .map_err(|_| AuthorizationError::ObjectParseError {
                        value: s.to_string(),
                    })?;
                Object::Customer(customer_ref)
            }
            Deposit => Object::Deposit,
            Withdraw => Object::Withdraw,
            Audit => Object::Audit,
            Ledger => Object::Ledger,
        };
        Ok(res)
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
