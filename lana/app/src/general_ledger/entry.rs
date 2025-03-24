use cala_ledger::{entry::Entry, EntryId};
use serde::{Deserialize, Serialize};

pub struct GeneralLedgerEntry {
    pub entry_id: EntryId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Entry> for GeneralLedgerEntry {
    fn from(entry: Entry) -> Self {
        Self {
            entry_id: entry.id,
            created_at: entry.created_at(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralLedgerEntryCursor {
    pub entry_id: EntryId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<&GeneralLedgerEntry> for GeneralLedgerEntryCursor {
    fn from(entry: &GeneralLedgerEntry) -> Self {
        Self {
            entry_id: entry.entry_id,
            created_at: entry.created_at,
        }
    }
}

impl From<cala_ledger::entry::EntriesByCreatedAtCursor> for GeneralLedgerEntryCursor {
    fn from(cursor: cala_ledger::entry::EntriesByCreatedAtCursor) -> Self {
        Self {
            entry_id: cursor.id,
            created_at: cursor.created_at,
        }
    }
}

impl From<GeneralLedgerEntryCursor> for cala_ledger::entry::EntriesByCreatedAtCursor {
    fn from(cursor: GeneralLedgerEntryCursor) -> Self {
        Self {
            id: cursor.entry_id,
            created_at: cursor.created_at,
        }
    }
}

impl async_graphql::connection::CursorType for GeneralLedgerEntryCursor {
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
