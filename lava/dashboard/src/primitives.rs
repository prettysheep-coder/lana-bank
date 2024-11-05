use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(
    async_graphql::Enum,
    Default,
    Debug,
    Deserialize,
    Clone,
    Copy,
    Serialize,
    Eq,
    PartialEq,
    sqlx::Type,
    Hash,
)]
#[sqlx(type_name = "TimeRange", rename_all = "snake_case")]
pub enum TimeRange {
    #[default]
    ThisQuarter,
    ThisYear,
}

impl TimeRange {
    pub fn all() -> &'static [TimeRange] {
        &[TimeRange::ThisQuarter, TimeRange::ThisYear]
    }

    pub fn in_same_range(&self, first_time: DateTime<Utc>, second_time: DateTime<Utc>) -> bool {
        match self {
            TimeRange::ThisQuarter => {
                let (year1, quarter1) = (first_time.year(), (first_time.month() - 1) / 3);
                let (year2, quarter2) = (second_time.year(), (second_time.month() - 1) / 3);
                year1 == year2 && quarter1 == quarter2
            }
            TimeRange::ThisYear => first_time.year() == second_time.year(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_range() {
        let range = TimeRange::ThisQuarter;
        let now = Utc::now();
        assert!(range.in_same_range(now, now));
        let latest = now + chrono::Duration::weeks(15);
        assert!(!range.in_same_range(now, latest));
    }
}
