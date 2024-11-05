use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use std::{fmt::Display, str::FromStr};

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

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum DashboardModuleAction {
    Dashboard(DashboardAction),
}

impl DashboardModuleAction {
    pub const DASHBOARD_READ: Self = DashboardModuleAction::Dashboard(DashboardAction::Read);
}

impl Display for DashboardModuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", DashboardModuleActionDiscriminants::from(self))?;
        use DashboardModuleAction::*;
        match self {
            Dashboard(action) => action.fmt(f),
        }
    }
}

impl FromStr for DashboardModuleAction {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (entity, action) = s.split_once(':').expect("missing colon");
        use DashboardModuleActionDiscriminants::*;
        let res = match entity.parse()? {
            Dashboard => DashboardAction::from(action.parse::<DashboardAction>()?),
        };
        Ok(res.into())
    }
}

#[derive(PartialEq, Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum DashboardAction {
    Read,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum DashboardModuleObject {
    Dashboard,
}

impl From<DashboardAction> for DashboardModuleAction {
    fn from(action: DashboardAction) -> Self {
        DashboardModuleAction::Dashboard(action)
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
