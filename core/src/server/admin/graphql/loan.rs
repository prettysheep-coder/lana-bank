use async_graphql::*;

use crate::server::shared_graphql::{
    loan::*,
    primitives::{Satoshis, UsdCents, UUID},
    terms::*,
};

#[derive(InputObject)]
pub struct LoanCreateInput {
    pub user_id: UUID,
    pub desired_principal: UsdCents,
    pub loan_terms: TermsInput,
}

#[derive(InputObject)]
pub struct TermsInput {
    pub annual_rate: AnnualRate,
    pub interval: InterestInterval,
    pub liquidation_cvl: CVLPct,
    pub duration: DurationInput,
    pub margin_call_cvl: CVLPct,
    pub initial_cvl: CVLPct,
}

#[derive(InputObject)]
pub(super) struct DurationInput {
    pub period: Period,
    pub units: u32,
}

#[derive(SimpleObject)]
pub struct LoanCreatePayload {
    loan: Loan,
}

impl From<crate::loan::Loan> for LoanCreatePayload {
    fn from(loan: crate::loan::Loan) -> Self {
        Self { loan: loan.into() }
    }
}

#[derive(InputObject)]
pub struct LoanPartialPaymentInput {
    pub loan_id: UUID,
    pub amount: UsdCents,
}

#[derive(SimpleObject)]
pub struct LoanPartialPaymentPayload {
    loan: Loan,
}

impl From<crate::loan::Loan> for LoanPartialPaymentPayload {
    fn from(loan: crate::loan::Loan) -> Self {
        Self { loan: loan.into() }
    }
}

#[derive(InputObject)]
pub struct LoanApproveInput {
    pub loan_id: UUID,
    pub collateral: Satoshis,
}

#[derive(SimpleObject)]
pub struct LoanApprovePayload {
    loan: Loan,
}

impl From<crate::loan::Loan> for LoanApprovePayload {
    fn from(loan: crate::loan::Loan) -> Self {
        Self { loan: loan.into() }
    }
}

impl From<DurationInput> for crate::loan::Duration {
    fn from(loan_duration: DurationInput) -> Self {
        match loan_duration.period {
            Period::Months => Self::Months(loan_duration.units),
        }
    }
}
