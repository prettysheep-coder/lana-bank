use es_entity::{PaginatedQueryArgs, PaginatedQueryRet};

use super::*;

pub struct CreditFacilitiesForSubject<'a> {
    subject: &'a Subject,
    authz: &'a Authorization,
}

impl<'a> CreditFacilitiesForSubject<'a> {
    pub(super) fn new(subject: &'a Subject, authz: &'a Authorization) -> Self {
        Self { subject, authz }
    }

    pub async fn list_by_created_at(
        &self,
        query: PaginatedQueryArgs<CreditFacilitiesByCreatedAtCursor>,
        direction: ListDirection,
    ) -> Result<
        PaginatedQueryRet<CreditFacility, CreditFacilitiesByCreatedAtCursor>,
        CreditFacilityError,
    > {
        self.authz.audit().audit(
            self.subject,
            Object::CreditFacility,
            CreditFacilityAction::List,
        )?;

        Ok(credit_facilities)
    }
}
