pub mod error;
mod terms;

use error::*;
use terms::*;

pub struct Loans {
    //
}

impl Loans {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_terms(&self, terms: NewTerms) -> Result<Terms, LoanError> {
        unimplemented!()
    }
}
