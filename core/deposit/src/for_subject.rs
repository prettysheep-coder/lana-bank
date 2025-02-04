use crate::{
    account::*, deposit_account_balance::*,
    deposit_account_cursor::DepositAccountsByCreatedAtCursor, error::*, ledger::*, primitives::*,
};

pub struct DepositsForSubject<'a> {
    subject: DepositAccountHolderId,
    accounts: &'a DepositAccountRepo,
    ledger: &'a DepositLedger,
}

impl<'a> DepositsForSubject<'a> {
    pub(super) fn new(
        subject: DepositAccountHolderId,
        accounts: &'a DepositAccountRepo,
        ledger: &'a DepositLedger,
    ) -> Self {
        Self {
            subject,
            accounts,
            ledger,
        }
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

    pub async fn account_balance(
        &self,
        account_id: impl Into<DepositAccountId> + std::fmt::Debug,
    ) -> Result<DepositAccountBalance, CoreDepositError> {
        let account_id = account_id.into();
        let balance = self.ledger.balance(account_id).await?;
        Ok(balance)
    }
}
