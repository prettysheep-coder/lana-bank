use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use es_entity::*;

use crate::primitives::{ApprovalProcessType, AuditInfo, CommitteeId, UserId};

#[derive(EsEvent, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "CommitteeId")]
pub enum CommitteeEvent {
    Initialized {
        id: CommitteeId,
        approval_process_type: ApprovalProcessType,
        audit_info: AuditInfo,
    },
    UserAdded {
        user_id: UserId,
        audit_info: AuditInfo,
    },
    UserRemoved {
        user_id: UserId,
        audit_info: AuditInfo,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EsEntityError"))]
pub struct Committee {
    pub id: CommitteeId,
    pub approval_process_type: ApprovalProcessType,
    pub(super) events: EntityEvents<CommitteeEvent>,
    pub audit_info: AuditInfo,
}

impl Committee {
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.events
            .entity_first_persisted_at()
            .expect("No events for deposit")
    }

    pub fn add_user(&mut self, user_id: UserId, audit_info: AuditInfo) {
        self.events.push(CommitteeEvent::UserAdded {
            user_id,
            audit_info,
        });
    }

    pub fn remove_user(&mut self, user_id: UserId, audit_info: AuditInfo) {
        self.events.push(CommitteeEvent::UserRemoved {
            user_id,
            audit_info,
        });
    }
}

impl TryFromEvents<CommitteeEvent> for Committee {
    fn try_from_events(events: EntityEvents<CommitteeEvent>) -> Result<Self, EsEntityError> {
        let mut builder = CommitteeBuilder::default();
        for event in events.iter_all() {
            match event {
                CommitteeEvent::Initialized {
                    id,
                    approval_process_type,
                    audit_info,
                    ..
                } => {
                    builder = builder
                        .id(*id)
                        .approval_process_type(*approval_process_type)
                        .audit_info(*audit_info)
                }
                CommitteeEvent::UserAdded { .. } => {}
                CommitteeEvent::UserRemoved { .. } => {}
            }
        }
        builder.events(events).build()
    }
}

#[derive(Debug, Builder)]
pub struct NewCommittee {
    #[builder(setter(into))]
    pub(super) id: CommitteeId,
    #[builder(setter(into))]
    pub(super) approval_process_type: ApprovalProcessType,
    #[builder(setter(into))]
    pub audit_info: AuditInfo,
}

impl NewCommittee {
    pub fn builder() -> NewCommitteeBuilder {
        NewCommitteeBuilder::default()
    }
}

impl IntoEvents<CommitteeEvent> for NewCommittee {
    fn into_events(self) -> EntityEvents<CommitteeEvent> {
        EntityEvents::init(
            self.id,
            [CommitteeEvent::Initialized {
                id: self.id,
                approval_process_type: self.approval_process_type,
                audit_info: self.audit_info,
            }],
        )
    }
}
