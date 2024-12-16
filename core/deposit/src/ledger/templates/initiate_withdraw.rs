use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use cala_ledger::{
    tx_template::{error::TxTemplateError, Params, *},
    *,
};

use crate::ledger::error::*;

pub const INITIATE_WITHDRAW_CODE: &str = "INITIATE_WITHDRAW_CODE";

#[derive(Debug, Serialize, Deserialize)]
pub struct InitiateWithdrawMeta {
    pub amount: Decimal,
    pub currency: Currency,
}

#[derive(Debug)]
pub struct InitiateWithdrawParams {
    pub journal_id: JournalId,
    pub deposit_omnibus_account_id: AccountId,
    pub credit_account_id: AccountId,
    pub meta: InitiateWithdrawMeta,
}

impl InitiateWithdrawParams {
    pub fn defs() -> Vec<NewParamDefinition> {
        vec![
            NewParamDefinition::builder()
                .name("journal_id")
                .r#type(ParamDataType::Uuid)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("currency")
                .r#type(ParamDataType::String)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("amount")
                .r#type(ParamDataType::Decimal)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("deposit_omnibus_account_id")
                .r#type(ParamDataType::Uuid)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("credit_account_id")
                .r#type(ParamDataType::Uuid)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("effective")
                .r#type(ParamDataType::Date)
                .build()
                .unwrap(),
            NewParamDefinition::builder()
                .name("meta")
                .r#type(ParamDataType::Json)
                .build()
                .unwrap(),
        ]
    }
}

impl From<InitiateWithdrawParams> for Params {
    fn from(
        InitiateWithdrawParams {
            journal_id,
            deposit_omnibus_account_id,
            credit_account_id,
            meta,
        }: InitiateWithdrawParams,
    ) -> Self {
        let mut params = Self::default();
        let amount = meta.amount;
        let currency = meta.currency;
        let meta = serde_json::to_value(meta).expect("Couldn't serialize meta");

        params.insert("journal_id", journal_id);
        params.insert("currency", currency);
        params.insert("amount", amount);
        params.insert("deposit_omnibus_account_id", deposit_omnibus_account_id);
        params.insert("credit_account_id", credit_account_id);
        params.insert("effective", chrono::Utc::now().date_naive());
        params.insert("meta", meta);

        params
    }
}

pub struct InitiateWithdraw;

impl InitiateWithdraw {
    #[instrument(name = "ledger.initiate_withdraw.init", skip_all)]
    pub async fn init(ledger: &CalaLedger) -> Result<(), DepositLedgerError> {
        let tx_input = NewTxTemplateTransaction::builder()
            .journal_id("params.journal_id")
            .effective("params.effective")
            .description("'Initiate a withdraw'")
            .metadata("params.meta")
            .build()
            .expect("Couldn't build TxInput");
        let entries = vec![
            NewTxTemplateEntry::builder()
                .entry_type("'INITIATE_WITHDRAW_SETTLED_DR'")
                .currency("params.currency")
                .account_id("params.deposit_omnibus_account_id")
                .direction("CREDIT")
                .layer("SETTLED")
                .units("params.amount")
                .build()
                .expect("Couldn't build entry"),
            NewTxTemplateEntry::builder()
                .entry_type("'INITIATE_WITHDRAW_SETTLED_CR'")
                .currency("params.currency")
                .account_id("params.credit_account_id")
                .direction("DEBIT")
                .layer("SETTLED")
                .units("params.amount")
                .build()
                .expect("Couldn't build entry"),
            NewTxTemplateEntry::builder()
                .entry_type("'INITIATE_WITHDRAW_PENDING_DR'")
                .currency("params.currency")
                .account_id("params.deposit_omnibus_account_id")
                .direction("DEBIT")
                .layer("PENDING")
                .units("params.amount")
                .build()
                .expect("Couldn't build entry"),
            NewTxTemplateEntry::builder()
                .entry_type("'INITIATE_WITHDRAW_PENDING_CR'")
                .currency("params.currency")
                .account_id("params.credit_account_id")
                .direction("CREDIT")
                .layer("PENDING")
                .units("params.amount")
                .build()
                .expect("Couldn't build entry"),
        ];

        let params = InitiateWithdrawParams::defs();
        let template = NewTxTemplate::builder()
            .id(TxTemplateId::new())
            .code(INITIATE_WITHDRAW_CODE)
            .transaction(tx_input)
            .entries(entries)
            .params(params)
            .build()
            .expect("Couldn't build template");
        match ledger.tx_templates().create(template).await {
            Err(TxTemplateError::DuplicateCode) => Ok(()),
            Err(e) => Err(e.into()),
            Ok(_) => Ok(()),
        }
    }
}
