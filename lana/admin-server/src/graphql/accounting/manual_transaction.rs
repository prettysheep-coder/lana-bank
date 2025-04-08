use crate::primitives::*;

use async_graphql::*;

// use lana_app::accounting::journal::JournalEntry;
pub use lana_app::accounting::manual_transactions::{
    ManualTransaction as DomainManualTransaction, ManualTransactionsByCreatedAtCursor,
};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ManualTransaction {
    id: ID,

    #[graphql(skip)]
    pub entity: Arc<DomainManualTransaction>,
}

#[ComplexObject]
impl ManualTransaction {
    // async fn entries(&self) -> Vec<JournalEntry> {
    // vec![]
    // }
}

impl From<DomainManualTransaction> for ManualTransaction {
    fn from(tx: DomainManualTransaction) -> Self {
        Self {
            id: tx.id.to_global_id(),
            entity: Arc::new(tx),
        }
    }
}

#[derive(InputObject)]
pub struct ManualTransactionExecuteInput {
    // pub deposit_account_id: UUID,
    // pub amount: UsdCents,
    pub reference: Option<String>,
}
crate::mutation_payload! { ManualTransactionExecutePayload, manual_transaction: ManualTransaction }
