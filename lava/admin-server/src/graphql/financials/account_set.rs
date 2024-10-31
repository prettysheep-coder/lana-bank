use async_graphql::*;

use crate::{graphql::account::*, primitives::*};

#[derive(SimpleObject)]
pub struct AccountSet {
    id: UUID,
    name: String,
    amounts: AccountAmountsByCurrency,
    has_sub_accounts: bool,
}

impl From<lava_app::ledger::account_set::LedgerAccountSetWithBalance> for AccountSet {
    fn from(line_item: lava_app::ledger::account_set::LedgerAccountSetWithBalance) -> Self {
        AccountSet {
            id: line_item.id.into(),
            name: line_item.name,
            amounts: line_item.balance.into(),
            has_sub_accounts: line_item.page_info.start_cursor.is_some(),
        }
    }
}

#[derive(Union)]
pub enum AccountSetSubAccount {
    Account(Account),
    AccountSet(AccountSet),
}

impl From<lava_app::ledger::account_set::PaginatedLedgerAccountSetSubAccountWithBalance>
    for AccountSetSubAccount
{
    fn from(
        member: lava_app::ledger::account_set::PaginatedLedgerAccountSetSubAccountWithBalance,
    ) -> Self {
        match member.value {
            lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance::Account(val) => {
                AccountSetSubAccount::Account(Account::from(val))
            }
            lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance::AccountSet(
                val,
            ) => AccountSetSubAccount::AccountSet(AccountSet::from(val)),
        }
    }
}

impl From<lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance>
    for AccountSetSubAccount
{
    fn from(member: lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance) -> Self {
        match member {
            lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance::Account(val) => {
                AccountSetSubAccount::Account(Account::from(val))
            }
            lava_app::ledger::account_set::LedgerAccountSetSubAccountWithBalance::AccountSet(
                val,
            ) => AccountSetSubAccount::AccountSet(AccountSet::from(val)),
        }
    }
}
