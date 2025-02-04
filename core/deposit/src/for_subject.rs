use crate::{
    account::*, deposit_account_cursor::DepositAccountsByCreatedAtCursor, error::*, primitives::*,
};

pub struct DepositsForSubject<'a> {
    subject: DepositAccountHolderId,
    accounts: &'a DepositAccountRepo,
}

impl<'a> DepositsForSubject<'a> {
    pub(super) fn new(subject: DepositAccountHolderId, accounts: &'a DepositAccountRepo) -> Self {
        Self { subject, accounts }
    }

    pub async fn list_accounts_by_created_at(
        &self,
        query: es_entity::PaginatedQueryArgs<DepositAccountsByCreatedAtCursor>,
        direction: impl Into<es_entity::ListDirection> + std::fmt::Debug,
    ) -> Result<
        es_entity::PaginatedQueryRet<DepositAccount, DepositAccountsByCreatedAtCursor>,
        CoreDepositError,
    > {
        Ok(self
            .accounts
            .list_for_account_holder_id_by_created_at(self.subject, query, direction.into())
            .await?)
    }
}
