use async_graphql::*;

use crate::primitives::*;

use super::deposit::Deposit;

#[derive(Union)]
pub enum DepositAccountHistoryEntry {
    Deposit(DepositEntry),
    Withdrawal(WithdrawalEntry),
    Disbursal(DisbursalEntry),
    Payment(PaymentEntry),
    Unknown(UnknownEntry),
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DepositEntry {
    #[graphql(skip)]
    pub tx_id: UUID,
    pub recorded_at: Timestamp,
}

#[derive(SimpleObject)]
pub struct WithdrawalEntry {
    pub tx_id: UUID,
    pub entry_id: UUID,
    pub recorded_at: Timestamp,
}

#[derive(SimpleObject)]
pub struct DisbursalEntry {
    pub tx_id: UUID,
    pub entry_id: UUID,
    pub recorded_at: Timestamp,
}

#[derive(SimpleObject)]
pub struct PaymentEntry {
    pub tx_id: UUID,
    pub entry_id: UUID,
    pub recorded_at: Timestamp,
}

#[derive(SimpleObject)]
pub struct UnknownEntry {
    pub tx_id: UUID,
    pub entry_id: UUID,
    pub recorded_at: Timestamp,
}

#[ComplexObject]
impl DepositEntry {
    async fn deposit(&self, ctx: &Context<'_>) -> async_graphql::Result<Deposit> {
        let (app, sub) = crate::app_and_sub_from_ctx!(ctx);

        let deposit = app
            .deposits()
            .for_subject(sub)?
            .find_deposit_by_id(self.tx_id)
            .await?;

        Ok(Deposit::from(deposit))
    }
}

impl From<lana_app::deposit::DepositAccountHistoryEntry> for DepositAccountHistoryEntry {
    fn from(entry: lana_app::deposit::DepositAccountHistoryEntry) -> Self {
        match entry {
            lana_app::deposit::DepositAccountHistoryEntry::Deposit(entry) => {
                Self::Deposit(DepositEntry {
                    tx_id: UUID::from(entry.tx_id),
                    recorded_at: entry.recorded_at.into(),
                })
            }
            lana_app::deposit::DepositAccountHistoryEntry::Withdrawal(entry) => {
                Self::Withdrawal(WithdrawalEntry {
                    tx_id: UUID::from(entry.tx_id),
                    entry_id: UUID::from(entry.entry_id),
                    recorded_at: entry.recorded_at.into(),
                })
            }
            lana_app::deposit::DepositAccountHistoryEntry::Disbursal(entry) => {
                Self::Disbursal(DisbursalEntry {
                    tx_id: UUID::from(entry.tx_id),
                    entry_id: UUID::from(entry.entry_id),
                    recorded_at: entry.recorded_at.into(),
                })
            }
            lana_app::deposit::DepositAccountHistoryEntry::Payment(entry) => {
                Self::Payment(PaymentEntry {
                    tx_id: UUID::from(entry.tx_id),
                    entry_id: UUID::from(entry.entry_id),
                    recorded_at: entry.recorded_at.into(),
                })
            }
            lana_app::deposit::DepositAccountHistoryEntry::Unknown(entry) => {
                Self::Unknown(UnknownEntry {
                    tx_id: UUID::from(entry.tx_id),
                    entry_id: UUID::from(entry.entry_id),
                    recorded_at: entry.recorded_at.into(),
                })
            }
            lana_app::deposit::DepositAccountHistoryEntry::Ignored => {
                unreachable!("Ignored entries should not be returned to the client")
            }
        }
    }
}
