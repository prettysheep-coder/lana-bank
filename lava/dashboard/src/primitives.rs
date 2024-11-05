use serde::{Deserialize, Serialize};

#[derive(async_graphql::Enum, Debug, Deserialize, Clone, Copy, Serialize, Eq, PartialEq)]
pub enum TimeRange {
    LastQuarter,
    LastYear,
}
