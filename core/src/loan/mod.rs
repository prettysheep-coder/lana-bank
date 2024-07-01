mod entity;
pub mod error;
mod repo;
mod terms;

use error::*;
use repo::*;
use terms::*;

pub struct Loans {
    pub loan_repo: LoanRepo,
    pub term_repo: TermRepo,
}

impl Loans {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        let loan_repo = LoanRepo::new(pool);
        let term_repo = TermRepo::new(pool);
        Self {
            loan_repo,
            term_repo,
        }
    }

    pub async fn update_current_terms(&self, terms: TermValues) -> Result<Terms, LoanError> {
        let terms = self.term_repo.update_current(terms).await?;
        Ok(terms)
    }
}
