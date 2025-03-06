use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use audit::AuditInfo;

use es_entity::*;

use super::primitives::*;
use super::tree;

#[derive(EsEvent, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "AltChartId")]
pub enum AltChartEvent {
    Initialized {
        id: AltChartId,
        name: String,
        reference: String,
        audit_info: AuditInfo,
    },
    NodeAdded {
        spec: AccountSpec,
        ledger_account_set_id: LedgerAccountSetId,
        audit_info: AuditInfo,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EsEntityError"))]
pub struct AltChart {
    pub id: AltChartId,
    pub reference: String,
    pub name: String,
    all_accounts: HashMap<AccountCode, (AccountSpec, LedgerAccountSetId)>,
    pub(super) events: EntityEvents<AltChartEvent>,
}

impl AltChart {
    pub fn create_node(
        &mut self,
        spec: &AccountSpec,
        audit_info: AuditInfo,
    ) -> Idempotent<(Option<LedgerAccountSetId>, LedgerAccountSetId)> {
        if self.all_accounts.contains_key(&spec.code) {
            return Idempotent::AlreadyApplied;
        }
        let ledger_account_set_id = LedgerAccountSetId::new();
        self.events.push(AltChartEvent::NodeAdded {
            spec: spec.clone(),
            ledger_account_set_id,
            audit_info,
        });
        let parent = if let Some(parent) = spec.parent.as_ref() {
            self.all_accounts.get(parent).map(|(_, id)| *id)
        } else {
            None
        };
        Idempotent::Executed((parent, ledger_account_set_id))
    }

    pub fn chart(&self) -> tree::ChartTree {
        tree::project(self.events.iter_all())
    }
}

impl TryFromEvents<AltChartEvent> for AltChart {
    fn try_from_events(events: EntityEvents<AltChartEvent>) -> Result<Self, EsEntityError> {
        let mut builder = AltChartBuilder::default();
        let mut all_accounts = HashMap::new();
        for event in events.iter_all() {
            match event {
                AltChartEvent::Initialized {
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
                AltChartEvent::NodeAdded {
                    spec,
                    ledger_account_set_id,
                    ..
                } => {
                    all_accounts.insert(spec.code.clone(), (spec.clone(), *ledger_account_set_id));
                }
            }
        }
        builder.all_accounts(all_accounts).events(events).build()
    }
}

#[derive(Debug, Builder)]
pub struct NewAltChart {
    #[builder(setter(into))]
    pub(super) id: AltChartId,
    pub(super) name: String,
    pub(super) reference: String,
    #[builder(setter(into))]
    pub audit_info: AuditInfo,
}

impl NewAltChart {
    pub fn builder() -> NewAltChartBuilder {
        NewAltChartBuilder::default()
    }
}

impl IntoEvents<AltChartEvent> for NewAltChart {
    fn into_events(self) -> EntityEvents<AltChartEvent> {
        EntityEvents::init(
            self.id,
            [AltChartEvent::Initialized {
                id: self.id,
                name: self.name,
                reference: self.reference,
                audit_info: self.audit_info,
            }],
        )
    }
}
