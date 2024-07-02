mod entity;
pub mod error;
mod repo;
mod terms;

use sqlx::PgPool;

use entity::*;
use error::*;
use repo::*;
use terms::*;

pub struct Loans {
    loan_repo: LoanRepo,
    term_repo: TermRepo,
    pool: PgPool,
}

impl Loans {
    pub fn new(pool: &PgPool) -> Self {
        let loan_repo = LoanRepo::new(pool);
        let term_repo = TermRepo::new(pool);
        Self {
            loan_repo,
            term_repo,
            pool: pool.clone(),
        }
    }

    pub async fn update_current_terms(&self, terms: TermValues) -> Result<Terms, LoanError> {
        let terms = self.term_repo.update_current(terms).await?;
        Ok(terms)
    }

    pub async fn create_loan_for_user(&self, new_loan: NewLoan) -> Result<Loan, LoanError> {
        let mut tx = self.pool.begin().await?;
        let loan = self.loan_repo.create_in_tx(&mut tx, new_loan).await?;
        tx.commit().await?;
        Ok(loan)
    }
}
