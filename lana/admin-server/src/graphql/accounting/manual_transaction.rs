use async_graphql::*;

pub use lana_app::accounting::manual_transactions::{
    ManualEntryInput, ManualTransaction as DomainManualTransaction,
    ManualTransactionsByCreatedAtCursor,
};

use crate::{graphql::primitives::*, primitives::*};

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
pub struct ManualTransactionEntryInput {
    pub account_ref: String,
    pub amount: Decimal,
    pub currency: String,
    pub direction: String,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct ManualTransactionExecuteInput {
    pub description: String,
    pub reference: Option<String>,
    pub entries: Vec<ManualTransactionEntryInput>,
}
crate::mutation_payload! { ManualTransactionExecutePayload, manual_transaction: ManualTransaction }
