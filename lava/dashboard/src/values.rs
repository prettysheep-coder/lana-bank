use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use lava_events::*;

use crate::primitives::TimeRange;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct DashboardValues {
    pub range: TimeRange,
    pub active_facilities: u32,
    pub pending_facilities: u32,
    pub last_updated: DateTime<Utc>,
}

impl DashboardValues {
    pub fn new(range: TimeRange) -> Self {
        Self {
            range,
            ..Default::default()
        }
    }

    pub(crate) fn process_event(&mut self, recorded_at: DateTime<Utc>, event: &LavaEvent) -> bool {
        let mut reset = false;
        if !self.range.in_same_range(self.last_updated, recorded_at) {
            *self = Self::new(self.range);
            reset = true;
        }

        self.last_updated = recorded_at;
        let updated = match event {
            LavaEvent::Credit(CreditEvent::CreditFacilityCreated { .. }) => {
                self.pending_facilities += 1;
                true
            }
            _ => false,
        };
        updated || reset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resets_when_event_is_not_in_same_range() {
        let mut dashboard = DashboardValues {
            pending_facilities: 3,
            last_updated: Utc::now() - chrono::Duration::weeks(15),
            ..Default::default()
        };
        dashboard.process_event(
            Utc::now(),
            &LavaEvent::Credit(CreditEvent::CreditFacilityCreated),
        );
        assert_eq!(dashboard.pending_facilities, 1);
    }
}
