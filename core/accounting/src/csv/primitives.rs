use serde::{Deserialize, Serialize};

use crate::primitives::AccountingCsvId;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, strum::Display, strum::EnumString, Copy,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountingCsvType {
    LedgerAccount,
    ProfitAndLoss,
    BalanceSheet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountingCsvStatus {
    Pending,
    Completed,
    Failed,
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
