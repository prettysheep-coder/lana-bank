use serde::{Deserialize, Serialize};

#[derive(
    async_graphql::Enum, Debug, Deserialize, Clone, Copy, Serialize, Eq, PartialEq, sqlx::Type,
)]
#[sqlx(type_name = "TimeRange", rename_all = "snake_case")]
pub enum TimeRange {
    ThisQuarter,
    ThisYear,
}
