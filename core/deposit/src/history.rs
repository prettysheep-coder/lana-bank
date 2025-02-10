use crate::primitives::LedgerTransactionId as LedgerTxId;

pub enum DepositAccountHistoryEntry {
    Deposit(LedgerTxId),
    Withdrawal(LedgerTxId),
    Disbursal(LedgerTxId),
    Unknown(LedgerTxId),
    Ignored,
}

const RECORD_DEPOSIT: &str = "RECORD_DEPOSIT_CR";
const INITIATE_WITHDRAW: &str = "INITIATE_WITHDRAW_SETTLED_DR";
const SETTLE_DISBURSAL: &str = "SETTLE_DISBURSAL_SETTLED_CR";

const IGNORE_INITIATE_WITHDRAW_PENDING: &str = "INITIATE_WITHDRAW_PENDING_CR";
const IGNORE_CONFIRM_WITHDRAWAL_PENDING: &str = "CONFIRM_WITHDRAW_PENDING_DR";

impl From<cala_ledger::entry::Entry> for DepositAccountHistoryEntry {
    fn from(entry: cala_ledger::entry::Entry) -> Self {
        match entry.values().entry_type.as_str() {
            RECORD_DEPOSIT => DepositAccountHistoryEntry::Deposit(entry.values().transaction_id),
            INITIATE_WITHDRAW => {
                DepositAccountHistoryEntry::Withdrawal(entry.values().transaction_id)
            }
            SETTLE_DISBURSAL => {
                DepositAccountHistoryEntry::Disbursal(entry.values().transaction_id)
            }

            IGNORE_CONFIRM_WITHDRAWAL_PENDING => DepositAccountHistoryEntry::Ignored,
            IGNORE_INITIATE_WITHDRAW_PENDING => DepositAccountHistoryEntry::Ignored,

            _ => DepositAccountHistoryEntry::Unknown(entry.values().transaction_id),
        }
    }
}
