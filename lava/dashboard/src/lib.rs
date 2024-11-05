#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod error;
mod primitives;
mod values;

use lava_events::LavaEvent;

use error::*;
pub use primitives::*;
pub use values::*;

type Outbox = outbox::Outbox<LavaEvent>;

pub struct Dashboard {
    _outbox: Outbox,
}

impl Dashboard {
    pub fn new(outbox: &Outbox) -> Self {
        Self {
            _outbox: outbox.clone(),
        }
    }

    pub async fn load_for_time_range(
        &self,
        _range: TimeRange,
    ) -> Result<DashboardValues, DashboardError> {
        let res = DashboardValues {
            active_facilities: 1,
            pending_facilities: 0,
        };
        Ok(res)
    }
}
