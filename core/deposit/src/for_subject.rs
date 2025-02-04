use crate::{
    account::*, deposit::*, deposit_account_balance::*,
    deposit_account_cursor::DepositAccountsByCreatedAtCursor, error::*, ledger::*, primitives::*,
};

pub struct DepositsForSubject<'a> {
    subject: DepositAccountHolderId,
    accounts: &'a DepositAccountRepo,
    deposits: &'a DepositRepo,
    ledger: &'a DepositLedger,
}

impl<'a> DepositsForSubject<'a> {
    pub(super) fn new(
        subject: DepositAccountHolderId,
        accounts: &'a DepositAccountRepo,
        deposits: &'a DepositRepo,
        ledger: &'a DepositLedger,
    ) -> Self {
        Self {
            subject,
            accounts,
            deposits,
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

        self.ensure_account_access(account_id).await?;

        let balance = self.ledger.balance(account_id).await?;
        Ok(balance)
    }

    pub async fn list_deposits_for_account(
        &self,
        account_id: impl Into<DepositAccountId> + std::fmt::Debug,
    ) -> Result<Vec<Deposit>, CoreDepositError> {
        let account_id = account_id.into();

        self.ensure_account_access(account_id).await?;

        Ok(self
            .deposits
            .list_for_deposit_account_id_by_created_at(
                account_id,
                Default::default(),
                es_entity::ListDirection::Descending,
            )
            .await?
            .entities)
    }

    async fn ensure_account_access(
        &self,
        account_id: DepositAccountId,
    ) -> Result<(), CoreDepositError> {
        let account = self.accounts.find_by_id(account_id).await?;

        if account.account_holder_id != self.subject {
            return Err(CoreDepositError::DepositAccountNotFound);
        }

        Ok(())
    }
}
