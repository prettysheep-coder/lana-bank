use chrono::{DateTime, Datelike, TimeZone, Utc};
use derive_builder::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::primitives::{Satoshis, UsdCents};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoanAnnualRate(Decimal);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoanLTVPct(Decimal);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LoanDuration {
    Months(u32),
}

impl LoanDuration {
    pub fn expiration_date(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            LoanDuration::Months(months) => start_date
                .checked_add_months(chrono::Months::new(*months))
                .expect("should return a expiration date"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InterestInterval {
    EndOfMonth,
}

impl InterestInterval {
    pub fn next_interest_payment(&self, current_date: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            InterestInterval::EndOfMonth => {
                let current_year = current_date.year();
                let current_month = current_date.month();

                let (year, month) = if current_month == 12 {
                    (current_year + 1, 1)
                } else {
                    (current_year, current_month + 1)
                };

                Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0)
                    .single()
                    .map(|dt| dt - chrono::Duration::seconds(1))
            }
        }
    }
}

#[derive(Builder, Debug, Serialize, Deserialize, Clone)]
pub struct TermValues {
    pub(crate) annual_rate: LoanAnnualRate,
    pub(crate) duration: LoanDuration,
    pub(crate) interval: InterestInterval,
    // overdue_penalty_rate: LoanAnnualRate,
    liquidation_ltv: LoanLTVPct,
    margin_call_ltv: LoanLTVPct,
    initial_ltv: LoanLTVPct,
}

impl TermValues {
    pub fn builder() -> TermValuesBuilder {
        TermValuesBuilder::default()
    }

    pub fn required_collateral(&self, _desired_principal: UsdCents) -> Satoshis {
        unimplemented!()
    }

    pub fn monthly_rate(&self) -> Decimal {
        self.annual_rate.0 / Decimal::from(12)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_interest_payment() {
        let interval = InterestInterval::EndOfMonth;
        let current_date = "2024-12-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let next_payment = "2024-12-31T23:59:59Z".parse::<DateTime<Utc>>().unwrap();

        assert_eq!(
            interval.next_interest_payment(current_date),
            Some(next_payment)
        );
    }

    #[test]
    fn expiration_date() {
        let start_date = "2024-12-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let duration = LoanDuration::Months(3);
        let expiration_date = "2025-03-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        assert_eq!(duration.expiration_date(start_date), expiration_date);
    }
}
