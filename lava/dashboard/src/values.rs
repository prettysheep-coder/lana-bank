use lava_events::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DashboardValues {
    pub active_facilities: u32,
    pub pending_facilities: u32,
}

impl DashboardValues {
    pub(crate) fn process_event(&mut self, event: &LavaEvent) -> bool {
        match event {
            LavaEvent::Credit(CreditEvent::CreditFacilityCreated { .. }) => {
                self.pending_facilities += 1;
                true
            }
            _ => false,
        }
    }
}
