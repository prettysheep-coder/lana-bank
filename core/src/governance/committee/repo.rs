use sqlx::PgPool;

use es_entity::*;

use crate::{data_export::Export, primitives::CommitteeId};

use super::{entity::*, error::*};

#[derive(EsRepo, Clone)]
#[es_repo(entity = "Committee", err = "CommitteeError", columns(name = "String"))]
pub struct CommitteeRepo {
    pool: PgPool,
    export: Export,
}

impl CommitteeRepo {
    pub fn new(pool: &PgPool, export: &Export) -> Self {
        Self {
            pool: pool.clone(),
            export: export.clone(),
        }
    }
}
