use async_graphql::*;

pub use lana_app::accounting::ledger_transaction::LedgerTransaction as DomainLedgerTransaction;

use crate::primitives::*;

use super::JournalEntry;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct LedgerTransaction {
    id: ID,
    created_at: Timestamp,
    ledger_transaction_id: UUID,

    #[graphql(skip)]
    pub entity: Arc<DomainLedgerTransaction>,
}

#[ComplexObject]
impl LedgerTransaction {
    async fn description(&self) -> &Option<String> {
        &self.entity.description
    }

    async fn entries(&self) -> Vec<JournalEntry> {
        self.entity
            .entries
            .iter()
            .map(|e| {
                let entry = e.clone();
                JournalEntry::from(entry)
            })
            .collect()
    }
}

impl From<DomainLedgerTransaction> for LedgerTransaction {
    fn from(tx: DomainLedgerTransaction) -> Self {
        Self {
            id: tx.id.to_global_id(),
            created_at: tx.created_at.into(),
            ledger_transaction_id: tx.id.into(),
            entity: Arc::new(tx),
        }
    }
}
