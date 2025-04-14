use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use audit::AuditInfo;
use es_entity::*;

use crate::primitives::{AccountingCsvId, LedgerAccountId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, strum::Display, strum::EnumString)]
#[serde(rename_all = "snake_case")]
pub enum AccountingCsvType {
    LedgerAccount,
    ProfitAndLoss,
    BalanceSheet,
}

#[derive(Debug)]
pub struct AccountingCsvLocationInCloud {
    pub csv_type: AccountingCsvType,
    pub bucket: String,
    pub path_in_bucket: String,
}

impl<'a> From<&'a AccountingCsvLocationInCloud> for cloud_storage::LocationInCloud<'a> {
    fn from(meta: &'a AccountingCsvLocationInCloud) -> Self {
        cloud_storage::LocationInCloud {
            bucket: &meta.bucket,
            path_in_bucket: &meta.path_in_bucket,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountingCsvDownloadLink {
    pub csv_type: AccountingCsvType,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct GeneratedAccountingCsvDownloadLink {
    pub accounting_csv_id: AccountingCsvId,
    pub link: AccountingCsvDownloadLink,
}

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
    pub(super) events: EntityEvents<AccountingCsvEvent>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountingCsvStatus {
    Pending,
    Completed,
    Failed,
}

impl AccountingCsv {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.events
            .entity_first_persisted_at()
            .expect("entity_first_persisted_at not found")
    }

    pub fn csv_type(&self) -> AccountingCsvType {
        for e in self.events.iter_all() {
            if let AccountingCsvEvent::Initialized { csv_type, .. } = e {
                return csv_type.clone();
            }
        }
        unreachable!("AccountingCsv must have Initialized event")
    }

    pub fn ledger_account_id(&self) -> Option<LedgerAccountId> {
        for e in self.events.iter_all() {
            if let AccountingCsvEvent::Initialized {
                ledger_account_id, ..
            } = e
            {
                return *ledger_account_id;
            }
        }
        None
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

    pub fn last_error(&self) -> Option<String> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::UploadFailed { error, .. } = e {
                return Some(error.clone());
            }
        }
        None
    }

    pub fn file_uploaded(&mut self, path_in_bucket: String, bucket: String, audit_info: AuditInfo) {
        self.events.push(AccountingCsvEvent::FileUploaded {
            path_in_bucket,
            bucket,
            audit_info,
            recorded_at: Utc::now(),
        });
    }

    pub fn upload_failed(&mut self, error: String, audit_info: AuditInfo) {
        self.events.push(AccountingCsvEvent::UploadFailed {
            error,
            audit_info,
            recorded_at: Utc::now(),
        });
    }

    pub fn bucket(&self) -> Option<String> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded { bucket, .. } = e {
                return Some(bucket.clone());
            }
        }
        None
    }

    pub fn path_in_bucket(&self) -> Option<String> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded { path_in_bucket, .. } = e {
                return Some(path_in_bucket.clone());
            }
        }
        None
    }

    pub fn download_link(&self) -> Option<AccountingCsvLocationInCloud> {
        for e in self.events.iter_all().rev() {
            if let AccountingCsvEvent::FileUploaded {
                bucket,
                path_in_bucket,
                ..
            } = e
            {
                return Some(AccountingCsvLocationInCloud {
                    csv_type: self.csv_type(),
                    bucket: bucket.clone(),
                    path_in_bucket: path_in_bucket.clone(),
                });
            }
        }
        None
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
            if let AccountingCsvEvent::Initialized { id, .. } = event {
                builder = builder.id(*id);
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
