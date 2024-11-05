const BQ_TABLE_NAME: &str = "credit_facility_events";

use crate::{data_export::Export, outbox::Outbox};

use super::{entity::*, error::*};

pub struct CreditFacilityPublisher {
    export: Export,
    outbox: Outbox,
}

impl CreditFacilityPublisher {
    pub fn new(export: &Export, outbox: &Outbox) -> Self {
        Self {
            export: export.clone(),
            outbox: outbox.clone(),
        }
    }

    pub async fn publish(
        &self,
        db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _: &CreditFacility,
        events: impl Iterator<Item = &PersistedEvent<CreditFacilityEvent>> + Clone,
    ) -> Result<(), CreditFacilityError> {
        self.export
            .es_entity_export(db, BQ_TABLE_NAME, events.clone())
            .await?;
        Ok(())
    }
}
