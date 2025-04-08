use crate::primitives::*;

use async_graphql::*;

// use lana_app::accounting::journal::JournalEntry;
pub use lana_app::accounting::manual_transactions::{
    ManualEntryInput, ManualTransaction as DomainManualTransaction,
    ManualTransactionsByCreatedAtCursor,
};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ManualTransaction {
    id: ID,
    created_at: Timestamp,

    #[graphql(skip)]
    pub entity: Arc<DomainManualTransaction>,
}

#[ComplexObject]
impl ManualTransaction {
    async fn description(&self) -> &str {
        &self.entity.description
    }

    async fn reference(&self) -> &str {
        &self.entity.reference
    }
    // async fn entries(&self) -> Vec<JournalEntry> {
    // vec![]
    // }
}

impl From<DomainManualTransaction> for ManualTransaction {
    fn from(tx: DomainManualTransaction) -> Self {
        Self {
            id: tx.id.to_global_id(),
            created_at: tx.created_at().into(),
            entity: Arc::new(tx),
        }
    }
}

#[derive(InputObject)]
pub struct ManualTransactionExecuteInput {
    pub chart_ref: String,
    pub description: String,
    pub entries: Vec<ManualEntryInput>,
    pub reference: Option<String>,
}
crate::mutation_payload! { ManualTransactionExecutePayload, manual_transaction: ManualTransaction }
