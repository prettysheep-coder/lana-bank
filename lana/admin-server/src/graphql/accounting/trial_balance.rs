use async_graphql::{connection::*, *};

use lana_app::{accounting::LedgerAccountId, trial_balance::TrialBalanceAccountCursor};

use crate::{graphql::loader::CHART_REF, primitives::*};

use super::{LedgerAccount, LedgerAccountBalanceRange};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct TrialBalance {
    name: String,

    #[graphql(skip)]
    from: Timestamp,
    #[graphql(skip)]
    until: Timestamp,
    #[graphql(skip)]
    entity: Arc<lana_app::trial_balance::TrialBalanceRoot>,
}

#[ComplexObject]
impl TrialBalance {
    async fn total(&self) -> async_graphql::Result<LedgerAccountBalanceRange> {
        if let Some(balance) = self.entity.btc_balance_range.as_ref() {
            Ok(Some(balance).into())
        } else {
            Ok(self.entity.usd_balance_range.as_ref().into())
        }
    }

    pub async fn accounts(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<
        Connection<TrialBalanceAccountCursor, LedgerAccount, EmptyFields, EmptyFields>,
    > {
        let (app, sub) = crate::app_and_sub_from_ctx!(ctx);

        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let query_args = es_entity::PaginatedQueryArgs { first, after };
                let res = app
                    .accounting()
                    .trial_balances()
                    .trial_balance_accounts(sub, self.name.to_string(), query_args)
                    .await?;

                let mut account_ids = Vec::with_capacity(res.entities.len());
                let mut external_ids = Vec::with_capacity(res.entities.len());

                for (account_id, external_id) in res.entities {
                    account_ids.push(account_id);
                    external_ids.push(external_id);
                }

                let mut accounts: std::collections::HashMap<LedgerAccountId, LedgerAccount> = app
                    .accounting()
                    .find_all_ledger_accounts_with_ranged_balance(
                        CHART_REF.0,
                        account_ids.as_slice(),
                        self.from.into_inner(),
                        Some(self.until.into_inner()),
                    )
                    .await?;

                let mut connection = Connection::new(false, res.has_next_page);
                for (account_id, external_id) in
                    account_ids.into_iter().zip(external_ids.into_iter())
                {
                    if let Some(account) = accounts.remove(&account_id) {
                        connection.edges.push(Edge::new(
                            TrialBalanceAccountCursor::from((
                                account_id,
                                external_id.expect("external_id should exist"),
                            )),
                            account,
                        ));
                        continue;
                    }
                }
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

impl From<lana_app::trial_balance::TrialBalanceRoot> for TrialBalance {
    fn from(trial_balance: lana_app::trial_balance::TrialBalanceRoot) -> Self {
        TrialBalance {
            name: trial_balance.name.to_string(),
            from: trial_balance.from.into(),
            until: trial_balance
                .until
                .expect("Mandatory 'until' value missing")
                .into(),
            entity: Arc::new(trial_balance),
        }
    }
}
