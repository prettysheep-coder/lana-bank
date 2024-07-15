use async_graphql::*;

use crate::{
    loan::{LoanAnnualRate, LoanCVLPct},
    server::shared_graphql::terms::*,
};

#[derive(InputObject)]
pub(super) struct CurrentTermsUpdateInput {
    pub annual_rate: LoanAnnualRate,
    pub interval: InterestInterval,
    pub liquidation_cvl: LoanCVLPct,
    pub duration: LoanDurationInput,
    pub margin_call_cvl: LoanCVLPct,
    pub initial_cvl: LoanCVLPct,
}

#[derive(InputObject)]
pub(super) struct LoanDurationInput {
    pub period: Period,
    pub units: u32,
}

#[derive(SimpleObject)]
pub struct CurrentTermsUpdatePayload {
    pub terms: Terms,
}

impl From<crate::loan::Terms> for CurrentTermsUpdatePayload {
    fn from(terms: crate::loan::Terms) -> Self {
        Self {
            terms: terms.into(),
        }
    }
}

impl From<LoanDurationInput> for crate::loan::LoanDuration {
    fn from(loan_duration: LoanDurationInput) -> Self {
        match loan_duration.period {
            Period::Months => Self::Months(loan_duration.units),
        }
    }
}
