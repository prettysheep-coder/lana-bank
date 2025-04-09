mod error;

use async_graphql::*;

use cala_ledger::DebitOrCredit;
pub use lana_app::accounting::manual_transactions::{
    ManualEntryInput, ManualTransaction as DomainManualTransaction,
    ManualTransactionsByCreatedAtCursor,
};

use crate::{
    graphql::{loader::*, primitives::*},
    primitives::*,
};

use super::JournalEntry;
use error::ManualTransactionInputError;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ManualTransaction {
    id: ID,
    created_at: Timestamp,
    ledger_transaction_id: UUID,

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

    async fn entries(&self, ctx: &Context<'_>) -> Vec<JournalEntry> {
        let _loader = ctx.data_unchecked::<LanaDataLoader>();
        // let x = loader.load_one(self.entity.ledger_transaction_id).await.unwrap();

        vec![]
    }
}

impl From<DomainManualTransaction> for ManualTransaction {
    fn from(tx: DomainManualTransaction) -> Self {
        Self {
            id: tx.id.to_global_id(),
            created_at: tx.created_at().into(),
            ledger_transaction_id: tx.ledger_transaction_id.into(),
            entity: Arc::new(tx),
        }
    }
}

#[derive(InputObject)]
pub struct ManualTransactionEntryInput {
    pub account_ref: String,
    pub amount: Decimal,
    pub currency: String,
    pub direction: DebitOrCredit,
    pub description: Option<String>,
}

impl TryFrom<ManualTransactionEntryInput> for ManualEntryInput {
    type Error = ManualTransactionInputError;

    fn try_from(i: ManualTransactionEntryInput) -> Result<Self, Self::Error> {
        let mut builder = ManualEntryInput::builder();

        builder.currency(
            i.currency
                .parse()
                .map_err(|_| ManualTransactionInputError::CurrencyNotSupported(i.currency))?,
        );

        builder.account_id_or_code(
            i.account_ref
                .parse()
                .map_err(|_| ManualTransactionInputError::AccountIdOrCodeInvalid(i.account_ref))?,
        );

        builder.direction(i.direction).amount(i.amount.into());

        if let Some(description) = i.description {
            builder.description(description);
        }

        Ok(builder.build().expect("all fields provided"))
    }
}

#[derive(InputObject)]
pub struct ManualTransactionExecuteInput {
    pub description: String,
    pub reference: Option<String>,
    pub entries: Vec<ManualTransactionEntryInput>,
}
crate::mutation_payload! { ManualTransactionExecutePayload, manual_transaction: ManualTransaction }
