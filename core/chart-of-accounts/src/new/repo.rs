use sqlx::PgPool;

use es_entity::*;

use crate::chart_of_accounts::error::ChartError;

use super::{entity::*, primitives::AltChartId};

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "AltChart",
    err = "ChartError",
    columns(reference(ty = "String")),
    tbl_prefix = "core"
)]
pub struct ChartRepo {
    pool: PgPool,
}

impl ChartRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}
