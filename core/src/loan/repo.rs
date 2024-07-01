use sqlx::PgPool;

#[derive(Clone)]
pub struct LoanRepo {
    _pool: PgPool,
}

impl LoanRepo {
    pub(super) fn new(pool: &PgPool) -> Self {
        Self {
            _pool: pool.clone(),
        }
    }
}
