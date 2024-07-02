use async_graphql::*;

use crate::primitives::{KycLevel, UserId};
use crate::{app::LavaApp, ledger, server::shared_graphql::primitives::UUID};

use super::balance::UserBalance;
use super::fixed_term_loan::FixedTermLoan;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum GraphqlKycLevel {
    Zero,
    One,
    Two,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    user_id: UUID,
    btc_deposit_address: String,
    ust_deposit_address: String,
    email: String,
    level: GraphqlKycLevel,
    #[graphql(skip)]
    account_ids: ledger::user::UserLedgerAccountIds,
}

#[ComplexObject]
impl User {
    async fn balance(&self, ctx: &Context<'_>) -> async_graphql::Result<UserBalance> {
        let app = ctx.data_unchecked::<LavaApp>();
        let balance = app.ledger().get_user_balance(self.account_ids).await?;
        Ok(UserBalance::from(balance))
    }

    async fn loans(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<FixedTermLoan>> {
        let app = ctx.data_unchecked::<LavaApp>();

        let loans: Vec<FixedTermLoan> = app
            .fixed_term_loans()
            .list_for_user(UserId::from(&self.user_id))
            .await?
            .into_iter()
            .map(FixedTermLoan::from)
            .collect();

        Ok(loans)
    }
}

impl From<KycLevel> for GraphqlKycLevel {
    fn from(level: KycLevel) -> Self {
        match level {
            KycLevel::NotKyced => GraphqlKycLevel::Zero,
            KycLevel::Basic => GraphqlKycLevel::One,
            KycLevel::Advanced => GraphqlKycLevel::Two,
        }
    }
}

impl From<crate::user::User> for User {
    fn from(user: crate::user::User) -> Self {
        User {
            user_id: UUID::from(user.id),
            btc_deposit_address: user.account_addresses.btc_address,
            ust_deposit_address: user.account_addresses.tron_usdt_address,
            email: user.email,
            account_ids: user.account_ids,
            level: GraphqlKycLevel::from(user.level),
        }
    }
}
