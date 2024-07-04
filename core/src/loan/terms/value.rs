use chrono::{DateTime, Datelike, TimeZone, Utc};
use derive_builder::Builder;
use rust_decimal::{prelude::*, Decimal};
use serde::{Deserialize, Serialize};

use crate::primitives::{BtcPrice, Satoshis, UsdCents};

const NUMBER_OF_DAYS_IN_YEAR: u32 = 366;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoanAnnualRate(Decimal);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoanCVLPct(Decimal);

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
    pub fn next_interest_payment(&self, current_date: DateTime<Utc>) -> DateTime<Utc> {
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
                    .expect("should return a valid date time")
                    - chrono::Duration::seconds(1)
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
    liquidation_cvl: LoanCVLPct,
    margin_call_cvl: LoanCVLPct,
    initial_cvl: LoanCVLPct,
}

impl TermValues {
    pub fn builder() -> TermValuesBuilder {
        TermValuesBuilder::default()
    }

    pub fn required_collateral(&self, desired_principal: UsdCents, price: BtcPrice) -> Satoshis {
        let desired_principal = Decimal::from(desired_principal.into_inner());
        let initial_cvl = self.initial_cvl.0;
        let price = Decimal::from(price.into_inner());
        let collateral = ((initial_cvl * desired_principal) / (Decimal::from(100) * price))
            .round_dp_with_strategy(8, RoundingStrategy::AwayFromZero);

        Satoshis::from_btc(collateral)
    }

    fn daily_rate(&self) -> Decimal {
        self.annual_rate.0 / Decimal::from(NUMBER_OF_DAYS_IN_YEAR)
    }

    pub fn calculate_interest(&self, principal: UsdCents, days: impl Into<Decimal>) -> UsdCents {
        let principal = Decimal::from(principal.into_inner());
        let daily_rate = self.daily_rate();
        let interest = (daily_rate * principal * days.into()).ceil();

        UsdCents::from(
            interest
                .to_u64()
                .expect("interest amount should be a positive integer"),
        )
    }
}

#[cfg(test)]
mod test {
    use rust_decimal_macros::dec;

    use crate::primitives::DUMMY_BTC_PRICE;

    use super::*;

    fn terms() -> TermValues {
        TermValues::builder()
            .annual_rate(LoanAnnualRate(Decimal::new(12, 2)))
            .duration(LoanDuration::Months(3))
            .interval(InterestInterval::EndOfMonth)
            .liquidation_cvl(LoanCVLPct(Decimal::from(105)))
            .margin_call_cvl(LoanCVLPct(Decimal::from(125)))
            .initial_cvl(LoanCVLPct(Decimal::from(140)))
            .build()
            .expect("should build a valid term")
    }

    #[test]
    fn next_interest_payment() {
        let interval = InterestInterval::EndOfMonth;
        let current_date = "2024-12-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let next_payment = "2024-12-31T23:59:59Z".parse::<DateTime<Utc>>().unwrap();

        assert_eq!(interval.next_interest_payment(current_date), next_payment);
    }

    #[test]
    fn expiration_date() {
        let start_date = "2024-12-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let duration = LoanDuration::Months(3);
        let expiration_date = "2025-03-03T14:00:00Z".parse::<DateTime<Utc>>().unwrap();
        assert_eq!(duration.expiration_date(start_date), expiration_date);
    }

    #[test]
    fn interest_calculation() {
        let terms = terms();
        let principal = UsdCents::from(100000);
        let days = 23;
        let interest = terms.calculate_interest(principal, days);
        assert_eq!(interest, UsdCents::from(755));
    }

    #[test]
    fn required_collateral() {
        let terms = terms();
        let principal = UsdCents::from(100000);
        let interest = terms.required_collateral(principal, DUMMY_BTC_PRICE);
        let sats = Satoshis::from_btc(dec!(0.02333334));
        assert_eq!(interest, sats);
    }
}
