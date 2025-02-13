use sqlx::PgPool;

pub use es_entity::Sort;
use es_entity::*;

use crate::primitives::*;

use super::{entity::*, error::*};

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "Custodian",
    err = "CustodianError",
    columns(name(ty = "String", list_by),)
)]
pub struct CustodianRepo {
    pool: PgPool,
}

impl CustodianRepo {
    pub(super) fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}
