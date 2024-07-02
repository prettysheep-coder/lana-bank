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
        let row = sqlx::query!(
            r#"
            WITH updated AS (
                UPDATE loan_terms
                SET current = FALSE
                WHERE current IS TRUE
                RETURNING id
            )
            INSERT INTO loan_terms (current, values)
            VALUES (true, $1)
            RETURNING id, values
            "#,
            serde_json::to_value(&terms).expect("should serialize term values"),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Terms {
            id: LoanTermsId::from(row.id),
            values: serde_json::from_value(row.values).expect("should deserialize term values"),
        })
    }

    pub async fn current(&self) -> Result<Terms, TermError> {
        let row = sqlx::query!(
            r#"
            SELECT id, values
            FROM loan_terms
            WHERE current IS TRUE
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Terms {
            id: LoanTermsId::from(row.id),
            values: serde_json::from_value(row.values).expect("should deserialize term values"),
        })
    }
}
