pub use lana_app::primitives::{CustomerId, Subject};

#[derive(Debug, Clone)]
pub struct CustomerAuthContext {
    pub sub: Subject,
}

impl CustomerAuthContext {
    pub fn new(sub: impl Into<CustomerId>) -> Self {
        Self {
            sub: Subject::Customer(sub.into()),
        }
    }
}
