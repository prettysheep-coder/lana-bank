#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "module")]
pub enum LavaEvent {
    Governance(governance::GovernanceEvent),
}

impl From<governance::GovernanceEvent> for LavaEvent {
    fn from(event: governance::GovernanceEvent) -> Self {
        Self::Governance(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let val = serde_json::to_string(&LavaEvent::from(
            governance::GovernanceEvent::ApprovalProcessConcluded {
                id: shared_primitives::ApprovalProcessId::new(),
                approved: true,
            },
        ))
        .unwrap();
        dbg!(val);
        assert!(false);
    }
}
