use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    entity::*,
    primitives::{AuditInfo, CustomerId, DocumentId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DocumentEvent {
    Initialized {
        id: DocumentId,
        customer_id: CustomerId,
        audit_info: AuditInfo,
        filename: String,
    },
    // DownloadLinkGenerated {
    //     report_name: String,
    //     bucket: String,
    //     path_in_bucket: String,
    //     audit_info: AuditInfo,
    //     recorded_at: DateTime<Utc>,
    // },
}

impl EntityEvent for DocumentEvent {
    type EntityId = DocumentId;
    fn event_table_name() -> &'static str {
        "document_events"
    }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityError"))]
pub struct Document {
    pub id: DocumentId,
    pub customer_id: CustomerId,
    pub filename: String,
    pub audit_info: AuditInfo,
    pub(super) events: EntityEvents<DocumentEvent>,
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document {}, uid: {}", self.id, self.customer_id)
    }
}

impl Entity for Document {
    type Event = DocumentEvent;
}

impl Document {
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.events
            .entity_first_persisted_at
            .expect("No events for document")
    }
}

impl TryFrom<EntityEvents<DocumentEvent>> for Document {
    type Error = EntityError;

    fn try_from(events: EntityEvents<DocumentEvent>) -> Result<Self, Self::Error> {
        let mut builder = DocumentBuilder::default();
        for event in events.iter() {
            match event {
                DocumentEvent::Initialized {
                    id,
                    customer_id,
                    audit_info,
                    filename,
                } => {
                    builder = builder
                        .id(*id)
                        .customer_id(*customer_id)
                        .filename(filename.clone())
                        .audit_info(*audit_info);
                }
            }
        }
        builder.events(events).build()
    }
}

#[derive(Debug)]
pub struct NewDocument {
    pub(super) id: DocumentId,
    pub(super) customer_id: CustomerId,
    pub(super) filename: String,
    pub audit_info: AuditInfo,
}

impl NewDocument {
    pub fn new(
        id: impl Into<DocumentId>,
        customer_id: impl Into<CustomerId>,
        filename: impl Into<String>,
        audit_info: impl Into<AuditInfo>,
    ) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
            filename: filename.into(),
            audit_info: audit_info.into(),
        }
    }

    pub(super) fn initial_events(self) -> EntityEvents<DocumentEvent> {
        EntityEvents::init(
            self.id,
            [DocumentEvent::Initialized {
                id: self.id,
                customer_id: self.customer_id,
                audit_info: self.audit_info,
                filename: self.filename,
            }],
        )
    }
}
