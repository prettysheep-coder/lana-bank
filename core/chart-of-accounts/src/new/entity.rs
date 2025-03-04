use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use audit::AuditInfo;

use es_entity::*;

use super::primitives::*;

#[derive(EsEvent, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "ChartId")]
pub enum ChartEvent {
    Initialized {
        id: ChartId,
        name: String,
        reference: String,
        audit_info: AuditInfo,
    },
    ControlAccountAdded {
        category: AccountCategory,
        audit_info: AuditInfo,
    },
    ControlSubAccountAdded {
        category: AccountCategory,
        audit_info: AuditInfo,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EsEntityError"))]
pub struct Chart {
    pub id: ChartId,
    pub reference: String,
    pub name: String,
    pub(super) events: EntityEvents<ChartEvent>,
}

impl Chart {
    pub fn create_control_account(&mut self, category: AccountCategory, audit_info: AuditInfo) {
        self.events.push(ChartEvent::ControlAccountAdded {
            category,
            audit_info,
        });
    }

    pub fn create_control_sub_account(&mut self, category: AccountCategory, audit_info: AuditInfo) {
        self.events.push(ChartEvent::ControlSubAccountAdded {
            category,
            audit_info,
        });
    }
}

impl TryFromEvents<ChartEvent> for Chart {
    fn try_from_events(events: EntityEvents<ChartEvent>) -> Result<Self, EsEntityError> {
        let mut builder = ChartBuilder::default();
        for event in events.iter_all() {
            match event {
                ChartEvent::Initialized {
                    id,
                    reference,
                    name,
                    ..
                } => {
                    builder = builder
                        .id(*id)
                        .reference(reference.to_string())
                        .name(name.to_string())
                }
                _ => (),
            }
        }
        builder.events(events).build()
    }
}

#[derive(Debug, Builder)]
pub struct NewChart {
    #[builder(setter(into))]
    pub(super) id: ChartId,
    pub(super) name: String,
    pub(super) reference: String,
    #[builder(setter(into))]
    pub audit_info: AuditInfo,
}

impl NewChart {
    pub fn builder() -> NewChartBuilder {
        NewChartBuilder::default()
    }
}

impl IntoEvents<ChartEvent> for NewChart {
    fn into_events(self) -> EntityEvents<ChartEvent> {
        EntityEvents::init(
            self.id,
            [ChartEvent::Initialized {
                id: self.id,
                name: self.name,
                reference: self.reference,
                audit_info: self.audit_info,
            }],
        )
    }
}
