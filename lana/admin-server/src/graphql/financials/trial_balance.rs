use async_graphql::{types::connection::*, *};
use chrono::{DateTime, Utc};

use crate::{graphql::account::AccountAmountsByCurrency, primitives::*};
use lana_app::trial_balance::{
    AccountSetHistoryCursor, AccountSetHistoryEntry as DomainTrialBalanceHistoryEntry,
};

#[derive(SimpleObject)]
pub struct TrialBalance {
    name: String,
    total: AccountAmountsByCurrency,
    accounts: Vec<TrialBalanceAccount>,
}

#[derive(SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl TrialBalanceAccount {
    async fn history(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<
        Connection<AccountSetHistoryCursor, TrialBalanceHistoryEntry, EmptyFields, EmptyFields>,
    > {
        let (app, sub) = crate::app_and_sub_from_ctx!(ctx);
        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let after = after.or(self.until.map(AccountSetHistoryCursor::from));
                let query_args = es_entity::PaginatedQueryArgs { first, after };
                let res = app
                    .trial_balances()
                    .account_set_history(sub, self.id, query_args)
                    .await?;

                let mut connection = Connection::new(false, res.has_next_page);
                connection
                    .edges
                    .extend(res.entities.into_iter().map(|entry| {
                        let cursor = AccountSetHistoryCursor::from(&entry);
                        Edge::new(cursor, TrialBalanceHistoryEntry::from(entry))
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[derive(SimpleObject)]
pub struct TrialBalanceHistoryEntry {
    pub tx_id: UUID,
    pub entry_id: UUID,
    pub recorded_at: DateTime<Utc>,
}

impl From<DomainTrialBalanceHistoryEntry> for TrialBalanceHistoryEntry {
    fn from(entry: DomainTrialBalanceHistoryEntry) -> Self {
        Self {
            tx_id: entry.tx_id.into(),
            entry_id: entry.entry_id.into(),
            recorded_at: entry.recorded_at,
        }
    }
}
