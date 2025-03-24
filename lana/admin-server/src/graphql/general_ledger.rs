use async_graphql::*;

use lana_app::general_ledger::GeneralLedgerEntry as DomainGeneralLedgerEntry;

pub use lana_app::general_ledger::GeneralLedgerEntryCursor;

use crate::primitives::*;

#[derive(SimpleObject)]
pub struct GeneralLedgerEntry {
    id: ID,
    entry_id: UUID,
}

impl From<DomainGeneralLedgerEntry> for GeneralLedgerEntry {
    fn from(entry: DomainGeneralLedgerEntry) -> Self {
        GeneralLedgerEntry {
            id: ID::from(entry.entry_id),
            entry_id: UUID::from(entry.entry_id),
        }
    }
}
