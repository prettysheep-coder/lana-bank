#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod entity;
pub mod error;
mod event;
mod primitives;
mod repo;

use std::collections::HashMap;
use tracing::instrument;

use audit::AuditSvc;
use authz::{Authorization, PermissionCheck};
use core_user::Role;
use outbox::{Outbox, OutboxEventMarker};

pub use entity::Customer;
use entity::*;
use error::*;
pub use event::*;
pub use primitives::*;
pub use repo::{customer_cursor::*, CustomerRepo, CustomersSortBy, FindManyCustomers, Sort};

pub struct Customers<Audit, E>
where
    Audit: AuditSvc,
    E: OutboxEventMarker<CoreCustomerEvent>,
{
    authz: Authorization<Audit, Role>,
    outbox: Outbox<E>,
    repo: CustomerRepo,
}

impl<Audit, E> Clone for Customers<Audit, E>
where
    Audit: AuditSvc,
    E: OutboxEventMarker<CoreCustomerEvent>,
{
    fn clone(&self) -> Self {
        Self {
            authz: self.authz.clone(),
            outbox: self.outbox.clone(),
            repo: self.repo.clone(),
        }
    }
}

impl<Audit, E> Customers<Audit, E>
where
    Audit: AuditSvc,
    <Audit as AuditSvc>::Subject: From<CustomerId>,
    <Audit as AuditSvc>::Action: From<CoreCustomerAction>,
    <Audit as AuditSvc>::Object: From<CustomerObject>,
    E: OutboxEventMarker<CoreCustomerEvent>,
{
    pub fn new(
        pool: &sqlx::PgPool,
        authz: &Authorization<Audit, Role>,
        outbox: &Outbox<E>,
    ) -> Self {
        let repo = CustomerRepo::new(pool);
        Self {
            repo,
            authz: authz.clone(),
            outbox: outbox.clone(),
        }
    }

    pub async fn subject_can_create_customer(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        enforce: bool,
    ) -> Result<Option<AuditInfo>, CustomerError> {
        Ok(self
            .authz
            .evaluate_permission(
                sub,
                CustomerObject::all_customers(),
                CoreCustomerAction::CUSTOMER_CREATE,
                enforce,
            )
            .await?)
    }

    #[instrument(name = "customer.create_customer", skip(self), err)]
    pub async fn create(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        email: impl Into<String> + std::fmt::Debug,
        telegram_id: impl Into<String> + std::fmt::Debug,
    ) -> Result<Customer, CustomerError> {
        let audit_info = self
            .subject_can_create_customer(sub, true)
            .await?
            .expect("audit info missing");

        let email = email.into();
        let telegram_id = telegram_id.into();

        let new_customer = NewCustomer::builder()
            .id(CustomerId::new())
            .email(email.clone())
            .telegram_id(telegram_id)
            .audit_info(audit_info)
            .build()
            .expect("Could not build customer");

        let mut db = self.repo.begin_op().await?;
        let customer = self.repo.create_in_op(&mut db, new_customer).await?;

        self.outbox
            .publish_persisted(
                db.tx(),
                CoreCustomerEvent::CustomerCreated {
                    id: customer.id,
                    email,
                },
            )
            .await?;

        db.commit().await?;

        Ok(customer)
    }

    #[instrument(name = "customer.create_customer", skip(self), err)]
    pub async fn find_by_id(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        id: impl Into<CustomerId> + std::fmt::Debug,
    ) -> Result<Option<Customer>, CustomerError> {
        let id = id.into();
        self.authz
            .enforce_permission(
                sub,
                CustomerObject::customer(id),
                CoreCustomerAction::CUSTOMER_READ,
            )
            .await?;

        match self.repo.find_by_id(id).await {
            Ok(customer) => Ok(Some(customer)),
            Err(e) if e.was_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[instrument(name = "customer.find_by_email", skip(self), err)]
    pub async fn find_by_email(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        email: String,
    ) -> Result<Option<Customer>, CustomerError> {
        self.authz
            .enforce_permission(
                sub,
                CustomerObject::all_customers(),
                CoreCustomerAction::CUSTOMER_READ,
            )
            .await?;

        match self.repo.find_by_email(email).await {
            Ok(customer) => Ok(Some(customer)),
            Err(e) if e.was_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_id_internal(
        &self,
        id: impl Into<CustomerId> + std::fmt::Debug,
    ) -> Result<Option<Customer>, CustomerError> {
        match self.repo.find_by_id(id.into()).await {
            Ok(customer) => Ok(Some(customer)),
            Err(e) if e.was_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn list(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        query: es_entity::PaginatedQueryArgs<CustomersCursor>,
        filter: FindManyCustomers,
        sort: impl Into<Sort<CustomersSortBy>>,
    ) -> Result<es_entity::PaginatedQueryRet<Customer, CustomersCursor>, CustomerError> {
        self.authz
            .enforce_permission(
                sub,
                CustomerObject::all_customers(),
                CoreCustomerAction::CUSTOMER_LIST,
            )
            .await?;
        self.repo.find_many(filter, sort.into(), query).await
    }

    pub async fn start_kyc(
        &self,
        db: &mut es_entity::DbOp<'_>,
        customer_id: CustomerId,
        applicant_id: String,
    ) -> Result<Customer, CustomerError> {
        let mut customer = self.repo.find_by_id(customer_id).await?;

        let audit_info = self
            .authz
            .audit()
            .record_system_entry_in_tx(
                db.tx(),
                CustomerObject::customer(customer_id),
                CoreCustomerAction::CUSTOMER_START_KYC,
            )
            .await?;

        customer.start_kyc(applicant_id, audit_info);

        self.repo.update_in_op(db, &mut customer).await?;

        Ok(customer)
    }

    pub async fn approve_basic(
        &self,
        db: &mut es_entity::DbOp<'_>,
        customer_id: CustomerId,
        applicant_id: String,
    ) -> Result<Customer, CustomerError> {
        let mut customer = self.repo.find_by_id(customer_id).await?;

        let audit_info = self
            .authz
            .audit()
            .record_system_entry_in_tx(
                db.tx(),
                CustomerObject::customer(customer_id),
                CoreCustomerAction::CUSTOMER_APPROVE_KYC,
            )
            .await?;

        customer.approve_kyc(KycLevel::Basic, applicant_id, audit_info);

        self.repo.update_in_op(db, &mut customer).await?;

        Ok(customer)
    }

    pub async fn deactivate(
        &self,
        db: &mut es_entity::DbOp<'_>,
        customer_id: CustomerId,
        applicant_id: String,
    ) -> Result<Customer, CustomerError> {
        let mut customer = self.repo.find_by_id(customer_id).await?;

        let audit_info = self
            .authz
            .audit()
            .record_system_entry_in_tx(
                db.tx(),
                CustomerObject::customer(customer_id),
                CoreCustomerAction::CUSTOMER_DECLINE_KYC,
            )
            .await?;

        customer.deactivate(applicant_id, audit_info);
        self.repo.update_in_op(db, &mut customer).await?;

        Ok(customer)
    }

    pub async fn find_all<T: From<Customer>>(
        &self,
        ids: &[CustomerId],
    ) -> Result<HashMap<CustomerId, T>, CustomerError> {
        self.repo.find_all(ids).await
    }

    #[instrument(name = "customer.update", skip(self), err)]
    pub async fn update(
        &self,
        sub: &<Audit as AuditSvc>::Subject,
        customer_id: impl Into<CustomerId> + std::fmt::Debug,
        new_telegram_id: String,
    ) -> Result<Customer, CustomerError> {
        let customer_id = customer_id.into();
        let audit_info = self
            .authz
            .enforce_permission(
                sub,
                CustomerObject::customer(customer_id),
                CoreCustomerAction::CUSTOMER_UPDATE,
            )
            .await?;

        let mut customer = self.repo.find_by_id(customer_id).await?;
        customer.update_telegram_id(new_telegram_id, audit_info);
        self.repo.update(&mut customer).await?;

        Ok(customer)
    }
}
