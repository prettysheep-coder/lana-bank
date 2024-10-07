mod entity;
mod error;
mod repo;

use error::DocumentError;
use repo::DocumentsRepo;

pub use entity::*;

use crate::{
    authorization::{Authorization, DocumentAction, Object},
    primitives::{CustomerId, DocumentId, Subject},
    storage::{ReportLocationInCloud, Storage},
};

#[derive(Clone)]
pub struct Documents {
    pool: sqlx::PgPool,
    authz: Authorization,
    storage: Storage,
    repo: DocumentsRepo,
}

impl Documents {
    pub fn new(pool: &sqlx::PgPool, storage: &Storage, authz: &Authorization) -> Self {
        Self {
            pool: pool.clone(),
            storage: storage.clone(),
            repo: DocumentsRepo::new(pool),
            authz: authz.clone(),
        }
    }

    pub async fn create(
        &self,
        sub: &Subject,
        content: Vec<u8>,
        customer_id: impl Into<CustomerId>,
        filename: String,
    ) -> Result<Document, DocumentError> {
        let customer_id = customer_id.into();

        let new_document_id = DocumentId::new();

        let audit_info = self
            .authz
            .enforce_permission(sub, Object::Document, DocumentAction::Create)
            .await?;

        let new_document = NewDocument::builder()
            .id(new_document_id)
            .customer_id(customer_id)
            .filename(filename)
            .audit_info(audit_info)
            .build()?;

        self
            .storage
            .upload(
                content,
                new_document.path_in_bucket().as_str(),
                "application/pdf",
            )
            .await?;

        let mut tx = self.pool.begin().await?;
        let document = self.repo.create_in_tx(&mut tx, new_document).await?;
        tx.commit().await?;
        Ok(document)
    }

    pub async fn find_by_id(
        &self,
        sub: &Subject,
        id: DocumentId,
    ) -> Result<Document, DocumentError> {
        self.authz
            .enforce_permission(sub, Object::Document, DocumentAction::Read)
            .await?;

        self.repo.find_by_id(id).await
    }

    pub async fn list_by_customer_id(
        &self,
        sub: &Subject,
        customer_id: CustomerId,
    ) -> Result<Vec<Document>, DocumentError> {
        self.authz
            .enforce_permission(sub, Object::Document, DocumentAction::List)
            .await?;

        self.repo.list_for_customer(customer_id).await
    }

    pub async fn generate_download_link(
        &self,
        sub: &Subject,
        document_id: DocumentId,
    ) -> Result<GeneratedDocumentDownloadLink, DocumentError> {
        let audit_info = self
            .authz
            .enforce_permission(sub, Object::Document, DocumentAction::GenerateDownloadLink)
            .await?;

        let mut document = self.repo.find_by_id(document_id).await?;

        let report_location_in_cloud = ReportLocationInCloud {
            bucket: self.storage.bucket_name(),
            path_in_bucket: document.path_in_bucket(),
        };

        let link = self
            .storage
            .generate_download_link(report_location_in_cloud)
            .await?;

        let mut tx = self.pool.begin().await?;

        document.download_link_generated(audit_info);

        self.repo.persist_in_tx(&mut tx, &mut document).await?;

        Ok(GeneratedDocumentDownloadLink { document_id, link })
    }
}
