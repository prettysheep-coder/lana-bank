use super::cala::graphql::*;

use crate::primitives::LedgerAccountSetId;

impl From<account_set_by_id::AccountSetByIdAccountSet> for LedgerAccountSetId {
    fn from(account_set: account_set_by_id::AccountSetByIdAccountSet) -> Self {
        LedgerAccountSetId::from(account_set.account_set_id)
    }
}
