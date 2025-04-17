pub mod error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use cala_ledger::{
    AccountSetId, BalanceId, CalaLedger, Currency, DebitOrCredit, JournalId, LedgerOperation,
    account_set::{
        AccountSet, AccountSetMemberByExternalId, AccountSetMemberId,
        AccountSetMembersByExternalIdCursor, NewAccountSet,
    },
};

use crate::primitives::{BalanceRange, CalaBalanceRange, LedgerAccountId};

use error::*;

#[derive(Clone)]
pub struct TrialBalanceRoot {
    pub id: AccountSetId,
    pub name: String,
    pub description: Option<String>,
    pub usd_balance_range: Option<BalanceRange>,
    pub btc_balance_range: Option<BalanceRange>,
    pub from: DateTime<Utc>,
    pub until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceAccountCursor {
    pub id: AccountSetId,
    pub external_id: String,
}

impl From<TrialBalanceAccountCursor> for AccountSetMembersByExternalIdCursor {
    fn from(cursor: TrialBalanceAccountCursor) -> Self {
        Self {
            id: AccountSetMemberId::AccountSet(cursor.id),
            external_id: Some(cursor.external_id),
        }
    }
}

impl From<AccountSetMembersByExternalIdCursor> for TrialBalanceAccountCursor {
    fn from(cursor: AccountSetMembersByExternalIdCursor) -> Self {
        let id = match cursor.id {
            AccountSetMemberId::AccountSet(id) => id,
            _ => panic!("Unexpected non-AccountSet cursor id found"),
        };
        Self {
            id,
            external_id: cursor.external_id.expect("external_id should exist"),
        }
    }
}

impl From<(LedgerAccountId, String)> for TrialBalanceAccountCursor {
    fn from((id, external_id): (LedgerAccountId, String)) -> Self {
        TrialBalanceAccountCursor {
            id: id.into(),
            external_id,
        }
    }
}

impl es_entity::graphql::async_graphql::connection::CursorType for TrialBalanceAccountCursor {
    type Error = String;

    fn encode_cursor(&self) -> String {
        use base64::{Engine as _, engine::general_purpose};
        let json = serde_json::to_string(&self).expect("could not serialize cursor");
        general_purpose::STANDARD_NO_PAD.encode(json.as_bytes())
    }

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        use base64::{Engine as _, engine::general_purpose};
        let bytes = general_purpose::STANDARD_NO_PAD
            .decode(s.as_bytes())
            .map_err(|e| e.to_string())?;
        let json = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

#[derive(Clone)]
pub struct TrialBalanceLedger {
    cala: CalaLedger,
    journal_id: JournalId,
}

impl TrialBalanceLedger {
    pub fn new(cala: &CalaLedger, journal_id: JournalId) -> Self {
        Self {
            cala: cala.clone(),
            journal_id,
        }
    }

    async fn create_unique_account_set(
        &self,
        op: &mut LedgerOperation<'_>,
        reference: &str,
        normal_balance_type: DebitOrCredit,
        parents: Vec<AccountSetId>,
    ) -> Result<AccountSetId, TrialBalanceLedgerError> {
        let id = AccountSetId::new();
        let new_account_set = NewAccountSet::builder()
            .id(id)
            .journal_id(self.journal_id)
            .external_id(reference)
            .name(reference)
            .description(reference)
            .normal_balance_type(normal_balance_type)
            .build()
            .expect("Could not build new account set");
        self.cala
            .account_sets()
            .create_in_op(op, new_account_set)
            .await?;

        for parent_id in parents {
            self.cala
                .account_sets()
                .add_member_in_op(op, parent_id, id)
                .await?;
        }

        Ok(id)
    }

    async fn get_account_set_with_balances(
        &self,
        account_set_id: AccountSetId,
        balances_by_id: &mut std::collections::HashMap<BalanceId, CalaBalanceRange>,
    ) -> Result<
        (
            AccountSet,
            (Option<CalaBalanceRange>, Option<CalaBalanceRange>),
        ),
        TrialBalanceLedgerError,
    > {
        let account_set = self.cala.account_sets().find(account_set_id).await?;

        let btc_balance =
            balances_by_id.remove(&(self.journal_id, account_set_id.into(), Currency::BTC));
        let usd_balance =
            balances_by_id.remove(&(self.journal_id, account_set_id.into(), Currency::USD));

        Ok((account_set, (btc_balance, usd_balance)))
    }

    async fn get_member_account_sets<U>(
        &self,
        account_set_id: AccountSetId,
        cursor: es_entity::PaginatedQueryArgs<U>,
    ) -> Result<
        es_entity::PaginatedQueryRet<AccountSetMemberByExternalId, U>,
        TrialBalanceLedgerError,
    >
    where
        U: std::fmt::Debug
            + From<AccountSetMembersByExternalIdCursor>
            + Into<AccountSetMembersByExternalIdCursor>,
    {
        let cala_cursor = es_entity::PaginatedQueryArgs {
            after: cursor.after.map(|u| u.into()),
            first: cursor.first,
        };

        let ret = self
            .cala
            .account_sets()
            .list_members_by_external_id(account_set_id, cala_cursor)
            .await?;

        Ok(es_entity::PaginatedQueryRet {
            entities: ret.entities,
            has_next_page: ret.has_next_page,
            end_cursor: ret.end_cursor.map(|c| c.into()),
        })
    }

    async fn get_balances_by_id(
        &self,
        all_account_set_ids: Vec<AccountSetId>,
        from: DateTime<Utc>,
        until: Option<DateTime<Utc>>,
    ) -> Result<std::collections::HashMap<BalanceId, CalaBalanceRange>, TrialBalanceLedgerError>
    {
        let balance_ids = all_account_set_ids
            .iter()
            .flat_map(|id| {
                [
                    (self.journal_id, (*id).into(), Currency::USD),
                    (self.journal_id, (*id).into(), Currency::BTC),
                ]
            })
            .collect::<Vec<_>>();
        let res = self
            .cala
            .balances()
            .find_all_in_range(&balance_ids, from, until)
            .await?;

        Ok(res)
    }

    pub async fn add_member(
        &self,
        op: es_entity::DbOp<'_>,
        node_account_set_id: impl Into<AccountSetId>,
        member: AccountSetId,
    ) -> Result<(), TrialBalanceLedgerError> {
        let mut op = self.cala.ledger_operation_from_db_op(op);
        self.add_member_in_op(&mut op, node_account_set_id, member)
            .await?;

        op.commit().await?;
        Ok(())
    }

    pub async fn add_members(
        &self,
        op: es_entity::DbOp<'_>,
        node_account_set_id: impl Into<AccountSetId> + Copy,
        members: impl Iterator<Item = AccountSetId>,
    ) -> Result<(), TrialBalanceLedgerError> {
        let mut op = self.cala.ledger_operation_from_db_op(op);
        for member in members {
            self.add_member_in_op(&mut op, node_account_set_id, member)
                .await?;
        }

        op.commit().await?;
        Ok(())
    }

    async fn add_member_in_op(
        &self,
        op: &mut LedgerOperation<'_>,
        node_account_set_id: impl Into<AccountSetId>,
        member: AccountSetId,
    ) -> Result<(), TrialBalanceLedgerError> {
        let node_account_set_id = node_account_set_id.into();

        match self
            .cala
            .account_sets()
            .add_member_in_op(op, node_account_set_id, member)
            .await
        {
            Ok(_) | Err(cala_ledger::account_set::error::AccountSetError::MemberAlreadyAdded) => {}
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }

    pub async fn create(
        &self,
        op: es_entity::DbOp<'_>,
        reference: &str,
    ) -> Result<AccountSetId, TrialBalanceLedgerError> {
        let mut op = self.cala.ledger_operation_from_db_op(op);

        let statement_id = self
            .create_unique_account_set(&mut op, reference, DebitOrCredit::Debit, vec![])
            .await?;

        op.commit().await?;
        Ok(statement_id)
    }

    pub async fn get_id_from_reference(
        &self,
        reference: String,
    ) -> Result<AccountSetId, TrialBalanceLedgerError> {
        Ok(self
            .cala
            .account_sets()
            .find_by_external_id(reference)
            .await?
            .id)
    }

    pub async fn get_trial_balance(
        &self,
        name: String,
        from: DateTime<Utc>,
        until: Option<DateTime<Utc>>,
    ) -> Result<TrialBalanceRoot, TrialBalanceLedgerError> {
        let statement_id = self.get_id_from_reference(name).await?;

        let mut balances_by_id = self
            .get_balances_by_id(vec![statement_id], from, until)
            .await?;

        let (account, balances) = self
            .get_account_set_with_balances(statement_id, &mut balances_by_id)
            .await?;

        Ok(TrialBalanceRoot::from((account, balances, from, until)))
    }

    pub async fn accounts(
        &self,
        name: String,
        query: es_entity::PaginatedQueryArgs<TrialBalanceAccountCursor>,
    ) -> Result<
        es_entity::PaginatedQueryRet<(LedgerAccountId, Option<String>), TrialBalanceAccountCursor>,
        TrialBalanceLedgerError,
    > {
        let statement_id = self.get_id_from_reference(name).await?;

        let member_account_sets = self
            .get_member_account_sets::<TrialBalanceAccountCursor>(statement_id, query)
            .await?;
        let member_account_sets_tuples = member_account_sets
            .entities
            .into_iter()
            .map(|m| match m.id {
                AccountSetMemberId::AccountSet(id) => Ok((id.into(), m.external_id)),
                _ => Err(TrialBalanceLedgerError::NonAccountSetMemberTypeFound),
            })
            .collect::<Result<Vec<(LedgerAccountId, Option<String>)>, TrialBalanceLedgerError>>()?;

        Ok(es_entity::PaginatedQueryRet {
            entities: member_account_sets_tuples,
            has_next_page: member_account_sets.has_next_page,
            end_cursor: member_account_sets.end_cursor,
        })
    }
}

impl
    From<(
        AccountSet,
        (Option<CalaBalanceRange>, Option<CalaBalanceRange>),
        DateTime<Utc>,
        Option<DateTime<Utc>>,
    )> for TrialBalanceRoot
{
    fn from(
        (account_set, (btc_balance, usd_balance), from, until): (
            AccountSet,
            (Option<CalaBalanceRange>, Option<CalaBalanceRange>),
            DateTime<Utc>,
            Option<DateTime<Utc>>,
        ),
    ) -> Self {
        let values = account_set.into_values();
        let usd_balance_range = usd_balance.map(|range| BalanceRange {
            start: Some(range.start),
            end: Some(range.end),
            diff: Some(range.diff),
        });
        let btc_balance_range = btc_balance.map(|range| BalanceRange {
            start: Some(range.start),
            end: Some(range.end),
            diff: Some(range.diff),
        });
        TrialBalanceRoot {
            id: values.id,
            name: values.name,
            description: values.description,
            btc_balance_range,
            usd_balance_range,
            from,
            until,
        }
    }
}
