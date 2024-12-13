use serde::{Deserialize, Serialize};

use governance::GovernanceEvent;
use outbox::OutboxEventMarker;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CoreDepositEvent {
    DepositAccountCreated,
    Governance(GovernanceEvent),
}

macro_rules! impl_event_marker {
    ($from_type:ty, $variant:ident) => {
        impl OutboxEventMarker<$from_type> for CoreDepositEvent {
            fn as_event(&self) -> Option<&$from_type> {
                match self {
                    Self::$variant(ref event) => Some(event),
                    _ => None,
                }
            }
        }
        impl From<$from_type> for CoreDepositEvent {
            fn from(event: $from_type) -> Self {
                Self::$variant(event)
            }
        }
    };
}

impl_event_marker!(GovernanceEvent, Governance);
