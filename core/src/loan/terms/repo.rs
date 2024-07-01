use sqlx::PgPool;

use crate::primitives::*;

use super::{error::TermError, TermValues, Terms};

#[derive(Clone)]
pub struct TermRepo {
    pool: PgPool,
}

impl TermRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn update_current(&self, terms: TermValues) -> Result<Terms, TermError> {
        let values = serde_json::to_value(&terms).expect("should serialize term values");
        let rows = sqlx::query!(
            r#"
            INSERT INTO loan_terms (values)
            VALUES ($1)
            RETURNING id, values
            "#,
            values,
        )
        .fetch_one(&self.pool)
        .await?;
        let values = serde_json::from_value(rows.values).expect("should deserialize term values");

        Ok(Terms {
            id: LoanTermsId::from(rows.id),
            values,
        })
    }
}
