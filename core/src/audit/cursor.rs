use async_graphql::types::connection::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditCursor {
    pub id: i64,
}

impl CursorType for AuditCursor {
    type Error = String;

    fn encode_cursor(&self) -> String {
        self.id.to_string()
    }

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        let id = s.parse::<i64>().map_err(|e| e.to_string())?;
        Ok(AuditCursor { id })
    }
}
