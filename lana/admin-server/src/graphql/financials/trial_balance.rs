use async_graphql::{types::connection::*, *};
use chrono::{DateTime, Utc};

use cala_ledger::DebitOrCredit;

use crate::{graphql::account::AccountAmountsByCurrency, primitives::*};

#[derive(SimpleObject)]
pub struct TrialBalance {
    name: String,
    total: AccountAmountsByCurrency,
    accounts: Vec<TrialBalanceAccount>,
}

#[derive(SimpleObject)]
pub struct TrialBalanceAccount {
    id: UUID,
    name: String,
    amounts: AccountAmountsByCurrency,

    #[graphql(skip)]
    until: Option<DateTime<Utc>>,
}

impl
    From<(
        lana_app::trial_balance::TrialBalanceAccountSet,
        Option<DateTime<Utc>>,
    )> for TrialBalanceAccount
{
    fn from(
        (line_item, until): (
            lana_app::trial_balance::TrialBalanceAccountSet,
            Option<DateTime<Utc>>,
        ),
    ) -> Self {
        TrialBalanceAccount {
            id: line_item.id.into(),
            name: line_item.name.to_string(),
            amounts: line_item.into(),
            until,
        }
    }
}

impl From<lana_app::trial_balance::TrialBalance> for TrialBalance {
    fn from(trial_balance: lana_app::trial_balance::TrialBalance) -> Self {
        TrialBalance {
            name: trial_balance.name.to_string(),
            total: trial_balance.clone().into(),
            accounts: trial_balance
                .accounts
                .into_iter()
                .map(|account| TrialBalanceAccount::from((account, trial_balance.until)))
                .collect(),
        }
    }
}
