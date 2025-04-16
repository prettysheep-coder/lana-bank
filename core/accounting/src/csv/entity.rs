use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use audit::AuditInfo;
use es_entity::*;

use crate::csv::primitives::{
    AccountingCsvLocationInCloud, AccountingCsvStatus, AccountingCsvType,
};
use crate::primitives::{AccountingCsvId, LedgerAccountId};

use super::error::AccountingCsvError;

#[derive(EsEvent, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[es_event(id = "AccountingCsvId")]
pub enum AccountingCsvEvent {
    Initialized {
        id: AccountingCsvId,
        csv_type: AccountingCsvType,
        ledger_account_id: Option<LedgerAccountId>,
        audit_info: AuditInfo,
    },
    FileUploaded {
        path_in_bucket: String,
        bucket: String,
        audit_info: AuditInfo,
        recorded_at: DateTime<Utc>,
    },
    UploadFailed {
        error: String,
        audit_info: AuditInfo,
        recorded_at: DateTime<Utc>,
    },
    DownloadLinkGenerated {
        bucket: String,
        path_in_bucket: String,
        audit_info: AuditInfo,
        recorded_at: DateTime<Utc>,
    },
}

#[derive(EsEntity, Builder)]
#[builder(pattern = "owned", build_fn(error = "EsEntityError"))]
pub struct AccountingCsv {
    pub id: AccountingCsvId,
    pub csv_type: AccountingCsvType,
    #[builder(setter(strip_option), default)]
    pub ledger_account_id: Option<LedgerAccountId>,
    pub(super) events: EntityEvents<AccountingCsvEvent>,
}

impl AccountingCsv {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.events
            .entity_first_persisted_at()
            .expect("entity_first_persisted_at not found")
    }

    pub fn status(&self) -> AccountingCsvStatus {
        for e in self.events.iter_all().rev() {
            match e {
                AccountingCsvEvent::FileUploaded { .. } => return AccountingCsvStatus::Completed,
                AccountingCsvEvent::UploadFailed { .. } => return AccountingCsvStatus::Failed,
                _ => {}
            }
        }
        AccountingCsvStatus::Pending
    }

    pub fn last_error(&self) -> Option<&str> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::UploadFailed { error, .. } = e {
                return Some(error);
            }
        }
        None
    }

    pub fn file_uploaded(
        &mut self,
        path_in_bucket: String,
        bucket: String,
        audit_info: AuditInfo,
    ) -> Idempotent<()> {
        idempotency_guard!(
            self.events.iter_all(),
            AccountingCsvEvent::FileUploaded { .. }
        );

        self.events.push(AccountingCsvEvent::FileUploaded {
            path_in_bucket,
            bucket,
            audit_info,
            recorded_at: Utc::now(),
        });
        Idempotent::Executed(())
    }

    pub fn upload_failed(&mut self, error: String, audit_info: AuditInfo) -> Idempotent<()> {
        idempotency_guard!(
            self.events.iter_all(),
            AccountingCsvEvent::UploadFailed { error: e, .. } if e == &error
        );

        self.events.push(AccountingCsvEvent::UploadFailed {
            error,
            audit_info,
            recorded_at: Utc::now(),
        });

        Idempotent::Executed(())
    }

    pub fn bucket(&self) -> Option<&str> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded { bucket, .. } = e {
                return Some(bucket);
            }
        }
        None
    }

    pub fn path_in_bucket(&self) -> Option<&str> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded { path_in_bucket, .. } = e {
                return Some(path_in_bucket);
            }
        }
        None
    }

    fn csv_uploaded(&self) -> Result<(), AccountingCsvError> {
        if self.status() != AccountingCsvStatus::Completed {
            return Err(AccountingCsvError::CsvNotReady);
        }
        Ok(())
    }

    pub fn location_in_cloud(&self) -> Result<AccountingCsvLocationInCloud, AccountingCsvError> {
        self.csv_uploaded()?;

        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded {
                bucket,
                path_in_bucket,
                ..
            } = e
            {
                return Ok(AccountingCsvLocationInCloud {
                    csv_type: self.csv_type,
                    bucket: bucket.clone(),
                    path_in_bucket: path_in_bucket.clone(),
                });
            }
        }
        Err(AccountingCsvError::CsvFileNotFound)
    }

    pub fn download_link_generated(
        &mut self,
        audit_info: AuditInfo,
        location: AccountingCsvLocationInCloud,
    ) {
        self.events.push(AccountingCsvEvent::DownloadLinkGenerated {
            bucket: location.bucket,
            path_in_bucket: location.path_in_bucket,
            audit_info,
            recorded_at: Utc::now(),
        });
    }
}

impl TryFromEvents<AccountingCsvEvent> for AccountingCsv {
    fn try_from_events(events: EntityEvents<AccountingCsvEvent>) -> Result<Self, EsEntityError> {
        let mut builder = AccountingCsvBuilder::default();

        for event in events.iter_all() {
            if let AccountingCsvEvent::Initialized {
                id,
                csv_type,
                ledger_account_id,
                ..
            } = event
            {
                builder = builder.id(*id).csv_type(*csv_type);
                if let Some(account_id) = ledger_account_id {
                    builder = builder.ledger_account_id(*account_id);
                }
            }
        }
        builder.events(events).build()
    }
}

#[derive(Builder, Debug)]
pub struct NewAccountingCsv {
    #[builder(setter(into))]
    pub(super) id: AccountingCsvId,
    #[builder(setter(into))]
    pub(super) csv_type: AccountingCsvType,
    #[builder(setter(strip_option), default)]
    pub(super) ledger_account_id: Option<LedgerAccountId>,
    #[builder(setter(into))]
    pub(super) audit_info: AuditInfo,
}
impl NewAccountingCsv {
    pub fn builder() -> NewAccountingCsvBuilder {
        NewAccountingCsvBuilder::default()
    }
}

impl IntoEvents<AccountingCsvEvent> for NewAccountingCsv {
    fn into_events(self) -> EntityEvents<AccountingCsvEvent> {
        EntityEvents::init(
            self.id,
            [AccountingCsvEvent::Initialized {
                id: self.id,
                csv_type: self.csv_type,
                ledger_account_id: self.ledger_account_id,
                audit_info: self.audit_info,
            }],
        )
    }
}
