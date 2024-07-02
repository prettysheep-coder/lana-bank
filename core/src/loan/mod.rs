mod entity;
pub mod error;
mod repo;
mod terms;

use sqlx::PgPool;

use crate::{job::JobRegistry, ledger::Ledger, primitives::*, user::Users};

use entity::*;
use error::*;
use repo::*;
use terms::*;

#[derive(Clone)]
pub struct Loans {
    loan_repo: LoanRepo,
    term_repo: TermRepo,
    users: Users,
    ledger: Ledger,
    pool: PgPool,
}

impl Loans {
    pub fn new(pool: &PgPool, _registry: &mut JobRegistry, users: &Users, ledger: &Ledger) -> Self {
        let loan_repo = LoanRepo::new(pool);
        let term_repo = TermRepo::new(pool);
        Self {
            loan_repo,
            term_repo,
            users: users.clone(),
            ledger: ledger.clone(),
            pool: pool.clone(),
        }
    }

    pub async fn update_current_terms(&self, terms: TermValues) -> Result<Terms, LoanError> {
        self.term_repo.update_current(terms).await
    }

    pub async fn create_loan_for_user(
        &self,
        user_id: UserId,
        desired_principal: UsdCents,
    ) -> Result<Loan, LoanError> {
        let current_terms = self.term_repo.find_current().await?;
        let user = match self.users.find_by_id(user_id).await? {
            Some(user) => user,
            None => return Err(LoanError::UserNotFound(user_id)),
        };

        if !user.may_create_loan() {
            return Err(LoanError::UserNotAllowedToCreateLoan(user_id));
        }
        let unallocated_collateral = self
            .ledger
            .get_user_balance(user.account_ids)
            .await?
            .btc_balance;

        let required_colateral = current_terms.required_colateral(desired_principal);

        if required_colateral > unallocated_collateral {
            return Err(LoanError::InsufficientCollateral(
                required_colateral,
                unallocated_collateral,
            ));
        }

        let new_loan = NewLoan::builder()
            .id(LoanId::new())
            .user_id(user_id)
            .terms(current_terms.values)
            .build()
            .expect("could not build new loan");
        let mut tx = self.pool.begin().await?;
        let loan = self.loan_repo.create_in_tx(&mut tx, new_loan).await?;
        tx.commit().await?;
        Ok(loan)
    }
}
