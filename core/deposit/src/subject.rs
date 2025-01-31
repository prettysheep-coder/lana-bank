
struct CoreDepositForSubject {
    pub async fn account_balance_for_subject(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
    ) -> Result<DepositAccountBalance, CoreDepositError>
    where
        <<Perms as PermissionCheck>::Audit as AuditSvc>::Subject: TryInto<DepositAccountHolderId>,
    {
        // self.authz.audit().record_entry(sub,
        //         CoreDepositObject::deposit_account(account_id),
        //         CoreDepositAction::DEPOSIT_ACCOUNT_READ_BALANCE,
        //         true,
        // )
        // let account_id = sub.into()
        unimplemented!()
    }
}

// -> SQL
// WHERE id = <>
// check if customer_id is as expected
