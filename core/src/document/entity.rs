use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    entity::*,
    primitives::{AuditInfo, CustomerId, DocumentId},
};

#[derive(Debug, Clone)]
pub struct GeneratedDocumentDownloadLink {
    pub document_id: DocumentId,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DocumentEvent {
    Initialized {
        id: DocumentId,
        customer_id: CustomerId,
        audit_info: AuditInfo,
        filename: String,
    },
    DownloadLinkGenerated {
        audit_info: AuditInfo,
    },
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

fn path_in_bucket_util(id: DocumentId) -> String {
    format!("documents/customer/{}", id)
}

impl Document {
    pub fn path_in_bucket(&self) -> String {
        path_in_bucket_util(self.id)
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.events
            .entity_first_persisted_at
            .expect("No events for document")
    }

    pub fn download_link_generated(&mut self, audit_info: AuditInfo) {
        self.events
            .push(DocumentEvent::DownloadLinkGenerated { audit_info });
    }
}

#[allow(clippy::single_match)]
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
                _ => (),
            }
        }
        builder.events(events).build()
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned", build_fn(error = "EntityError"))]
pub struct NewDocument {
    #[builder(setter(into))]
    pub(super) id: DocumentId,
    #[builder(setter(into))]
    pub(super) customer_id: CustomerId,
    #[builder(setter(custom))]
    pub(super) filename: String,
    #[builder(setter(into))]
    pub audit_info: AuditInfo,
}

impl NewDocumentBuilder {
    // Custom setter for filename to apply sanitization
    pub fn filename<T: Into<String>>(mut self, filename: T) -> Self {
        let sanitized = filename
            .into()
            .trim()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "-");
        self.filename = Some(sanitized);
        self
    }
}

impl NewDocument {
    pub fn builder() -> NewDocumentBuilder {
        NewDocumentBuilder::default()
    }

    pub fn path_in_bucket(&self) -> String {
        path_in_bucket_util(self.id)
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
