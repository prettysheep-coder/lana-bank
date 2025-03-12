use serde::{Deserialize, Serialize};

use crate::primitives::{LedgerEntryId, LedgerTxId};

pub struct TrialBalanceEntry {
    pub tx_id: LedgerTxId,
    pub entry_id: LedgerEntryId,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl From<cala_ledger::entry::Entry> for TrialBalanceEntry {
    fn from(cala_entry: cala_ledger::entry::Entry) -> Self {
        Self {
            tx_id: cala_entry.values().transaction_id,
            entry_id: cala_entry.id,
            recorded_at: cala_entry.created_at(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceEntryCursor {
    pub entry_id: LedgerEntryId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<TrialBalanceEntryCursor> for cala_ledger::entry::EntriesByCreatedAtCursor {
    fn from(cursor: TrialBalanceEntryCursor) -> Self {
        Self {
            id: cursor.entry_id,
            created_at: cursor.created_at,
        }
    }
}

impl From<cala_ledger::entry::EntriesByCreatedAtCursor> for TrialBalanceEntryCursor {
    fn from(cursor: cala_ledger::entry::EntriesByCreatedAtCursor) -> Self {
        Self {
            entry_id: cursor.id,
            created_at: cursor.created_at,
        }
    }
}

impl From<&TrialBalanceEntry> for TrialBalanceEntryCursor {
    fn from(entry: &TrialBalanceEntry) -> Self {
        Self {
            entry_id: entry.entry_id,
            created_at: entry.recorded_at,
        }
    }
}

mod graphql {
    use async_graphql::{connection::CursorType, *};

    use super::*;

    impl CursorType for TrialBalanceEntryCursor {
        type Error = String;

        fn encode_cursor(&self) -> String {
            use base64::{engine::general_purpose, Engine as _};
            let json = serde_json::to_string(&self).expect("could not serialize cursor");
            general_purpose::STANDARD_NO_PAD.encode(json.as_bytes())
        }

        fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
            use base64::{engine::general_purpose, Engine as _};
            let bytes = general_purpose::STANDARD_NO_PAD
                .decode(s.as_bytes())
                .map_err(|e| e.to_string())?;
            let json = String::from_utf8(bytes).map_err(|e| e.to_string())?;
            serde_json::from_str(&json).map_err(|e| e.to_string())
        }
    }
}
