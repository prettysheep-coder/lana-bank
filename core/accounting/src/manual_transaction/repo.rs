use sqlx::PgPool;

use es_entity::*;

use crate::primitives::{CalaTxId, ManualTransactionId};

use super::{entity::*, error::*};

#[derive(EsRepo)]
#[es_repo(
    entity = "ManualTransaction",
    err = "ManualTransactionError",
    columns(
        reference(ty = "String", create(accessor = "reference()")),
        ledger_transaction_id(ty = "CalaTxId")
    ),
    tbl_prefix = "core"
)]
pub struct ManualTransactionRepo {
    pool: PgPool,
}

impl Clone for ManualTransactionRepo {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl ManualTransactionRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}
