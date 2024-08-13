use async_graphql::*;

use crate::{
    app::LavaApp,
    ledger,
    primitives::{CustomerId, LoanStatus},
    server::shared_graphql::{customer::Customer, primitives::*, terms::TermValues},
};

use super::convert::ToGlobalId;

#[derive(SimpleObject)]
pub struct Transaction {
    amount: UsdCents,
    transaction_type: TransactionType,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    InterestPayment,
    PrincipalPayment,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Loan {
    id: ID,
    loan_id: UUID,
    created_at: Timestamp,
    loan_terms: TermValues,
    #[graphql(skip)]
    customer_id: UUID,
    #[graphql(skip)]
    account_ids: crate::ledger::loan::LoanAccountIds,
    status: LoanStatus,
    transactions: Vec<Transaction>,
}

#[ComplexObject]
impl Loan {
    async fn balance(&self, ctx: &Context<'_>) -> async_graphql::Result<LoanBalance> {
        let app = ctx.data_unchecked::<LavaApp>();
        let balance = app.ledger().get_loan_balance(self.account_ids).await?;
        Ok(LoanBalance::from(balance))
    }

    async fn customer(&self, ctx: &Context<'_>) -> async_graphql::Result<Customer> {
        let app = ctx.data_unchecked::<LavaApp>();
        let user = app
            .customers()
            .find_by_id(None, CustomerId::from(&self.customer_id))
            .await?;

        match user {
            Some(user) => Ok(Customer::from(user)),
            None => panic!("user not found for a loan. should not be possible"),
        }
    }
}

#[derive(SimpleObject)]
struct Collateral {
    btc_balance: Satoshis,
}

#[derive(SimpleObject)]
struct LoanOutstanding {
    usd_balance: UsdCents,
}

#[derive(SimpleObject)]
struct InterestIncome {
    usd_balance: UsdCents,
}

#[derive(SimpleObject)]
pub(super) struct LoanBalance {
    collateral: Collateral,
    outstanding: LoanOutstanding,
    interest_incurred: InterestIncome,
}

impl From<ledger::loan::LoanBalance> for LoanBalance {
    fn from(balance: ledger::loan::LoanBalance) -> Self {
        Self {
            collateral: Collateral {
                btc_balance: balance.collateral,
            },
            outstanding: LoanOutstanding {
                usd_balance: balance.principal_receivable + balance.interest_receivable,
            },
            interest_incurred: InterestIncome {
                usd_balance: balance.interest_incurred,
            },
        }
    }
}

impl ToGlobalId for crate::primitives::LoanId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("loan:{}", self))
    }
}

impl From<crate::loan::Loan> for Loan {
    fn from(loan: crate::loan::Loan) -> Self {
        let created_at = loan.created_at().into();
        let transactions = loan
            .transactions()
            .into_iter()
            .flat_map(|event| Vec::<Transaction>::from(event))
            .collect();
        Loan {
            id: loan.id.to_global_id(),
            loan_id: UUID::from(loan.id),
            customer_id: UUID::from(loan.customer_id),
            status: loan.status().into(),
            loan_terms: TermValues::from(loan.terms),
            account_ids: loan.account_ids,
            created_at,
            transactions,
        }
    }
}

impl From<&crate::loan::LoanEvent> for Vec<Transaction> {
    fn from(event: &crate::loan::LoanEvent) -> Self {
        match event {
            crate::loan::LoanEvent::PaymentRecorded {
                interest_amount,
                principal_amount,
                ..
            } => {
                let mut transactions = Vec::new();

                if *interest_amount != UsdCents::ZERO {
                    transactions.push(Transaction {
                        amount: *interest_amount,
                        transaction_type: TransactionType::InterestPayment,
                    });
                }

                if *principal_amount != UsdCents::ZERO {
                    transactions.push(Transaction {
                        amount: *principal_amount,
                        transaction_type: TransactionType::PrincipalPayment,
                    });
                }

                transactions
            }
            _ => unreachable!("unexpected event type in loan transaction list"),
        }
    }
}
