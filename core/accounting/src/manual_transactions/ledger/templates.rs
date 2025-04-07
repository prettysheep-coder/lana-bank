use std::num::NonZeroU8;

use cala_ledger::{
    TxTemplateId,
    tx_template::{
        NewParamDefinition, NewTxTemplate, ParamDataType, TxTemplate, TxTemplates,
        error::TxTemplateError,
    },
};

use crate::manual_transactions::error::ManualTransactionError;

/// Generates template code for manual transaction with `n_entries` entries.
fn template_code_for_n(n_entries: NonZeroU8) -> String {
    format!("MANUAL_TRANSACTION_FOR_{n_entries}")
}

#[derive(Clone)]
pub struct ManualTransactionTemplates {
    tx_templates: TxTemplates,
}

impl ManualTransactionTemplates {
    pub fn new(tx_templates: &TxTemplates) -> Self {
        Self {
            tx_templates: tx_templates.clone(),
        }
    }

    /// Returns a manual transaction template from underlying Cala for `n` entries.
    /// If no such template is found, it is created.
    pub async fn get_template_for_n_entries(
        &self,
        n_entries: NonZeroU8,
    ) -> Result<TxTemplate, ManualTransactionError> {
        let existing_template = self
            .tx_templates
            .find_by_code(template_code_for_n(n_entries))
            .await;

        let template = match existing_template {
            Ok(x) => Ok(x),
            Err(TxTemplateError::NotFound) => Ok(self
                .tx_templates
                .create(new_template_n_entries(n_entries))
                .await?),
            err => err,
        }?;

        Ok(template)
    }
}

fn new_template_n_entries(n: NonZeroU8) -> NewTxTemplate {
    let template = NewTxTemplate::builder()
        .id(TxTemplateId::new())
        .code(template_code_for_n(n))
        .params(params_for_n_entries(n))
        .build()
        .unwrap();

    unimplemented!()
}

fn params_for_n_entries(n: NonZeroU8) -> Vec<NewParamDefinition> {
    let mut params = vec![
        NewParamDefinition::builder()
            .name("journal_id")
            .r#type(ParamDataType::Uuid)
            .build()
            .unwrap(),
        NewParamDefinition::builder()
            .name("effective")
            .r#type(ParamDataType::Date)
            .build()
            .unwrap(),
    ];

    for i in 0..n.into() {
        [
            ("account_id", ParamDataType::String),
            ("direction", ParamDataType::String),
            ("currency", ParamDataType::String),
            ("amount", ParamDataType::Decimal),
            ("description", ParamDataType::String),
        ]
        .into_iter()
        .for_each(|(name, data_type)| {
            params.push(
                NewParamDefinition::builder()
                    .name(format!("entry_{i}_{name}"))
                    .r#type(data_type)
                    .build()
                    .unwrap(),
            );
        });
    }

    params
}
