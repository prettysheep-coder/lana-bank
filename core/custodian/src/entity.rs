use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use audit::AuditInfo;
use es_entity::*;

use crate::primitives::*;

#[derive(EsEvent, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "CustodianId")]
pub enum CustodianEvent {
    Initialized {
        id: CustodianId,
        name: String,
        audit_info: AuditInfo,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EsEntityError"))]
pub struct Custodian {
    pub id: CustodianId,
    pub name: String,
    pub(super) events: EntityEvents<CustodianEvent>,
}

impl TryFromEvents<CustodianEvent> for Custodian {
    fn try_from_events(events: EntityEvents<CustodianEvent>) -> Result<Self, EsEntityError> {
        let mut builder = CustodianBuilder::default();

        for event in events.iter_all() {
            match event {
                CustodianEvent::Initialized { id, name, .. } => {
                    builder = builder.id(*id).name(name.clone())
                }
            }
        }

        builder.events(events).build()
    }
}

#[derive(Debug, Builder)]
pub struct NewCustodian {
    #[builder(setter(into))]
    pub(super) id: CustodianId,
    #[builder(setter(into))]
    pub(super) name: String,
    pub(super) audit_info: AuditInfo,
}

impl NewCustodian {
    pub fn builder() -> NewCustodianBuilder {
        NewCustodianBuilder::default()
    }
}

impl IntoEvents<CustodianEvent> for NewCustodian {
    fn into_events(self) -> EntityEvents<CustodianEvent> {
        EntityEvents::init(
            self.id,
            [CustodianEvent::Initialized {
                id: self.id,
                name: self.name,
                audit_info: self.audit_info,
            }],
        )
    }
}
