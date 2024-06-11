use crate::primitives::LedgerAccountId;

use super::cala::graphql::*;

impl From<account_by_id::AccountByIdAccount> for LedgerAccountId {
    fn from(account: account_by_id::AccountByIdAccount) -> Self {
        Self::from(account.account_id)
    }
}
