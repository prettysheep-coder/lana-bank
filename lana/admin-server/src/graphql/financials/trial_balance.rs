use async_graphql::*;

use lana_app::accounting_init::constants::CHART_REF;

use crate::{
    app_and_sub_from_ctx,
    graphql::{account::AccountAmountsByCurrency, ledger_account::AccountCode},
    primitives::*,
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
}

#[ComplexObject]
impl TrialBalanceAccount {
    async fn code(&self, ctx: &Context<'_>) -> async_graphql::Result<AccountCode> {
        let reference = CHART_REF.to_string();
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let chart = app
            .chart_of_accounts()
            .find_by_reference(sub, reference.clone())
            .await?
            .unwrap_or_else(|| panic!("Chart of accounts not found for ref {}", reference));

        let code = app
            .chart_of_accounts()
            .account_details_by_id(sub, chart, self.id.into())
            .await?
            .code
            .to_string();

        Ok(AccountCode::from(code))
    }
}

impl From<lana_app::trial_balance::TrialBalanceAccountSet> for TrialBalanceAccount {
    fn from(line_item: lana_app::trial_balance::TrialBalanceAccountSet) -> Self {
        TrialBalanceAccount {
            id: line_item.id.into(),
            name: line_item.name.to_string(),
            amounts: line_item.into(),
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
                .map(TrialBalanceAccount::from)
                .collect(),
        }
    }
}
