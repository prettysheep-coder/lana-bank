use cala_ledger::{
    tx_template::{error::TxTemplateError, Params, *},
    *,
};

struct ManualTransactionTemplate {}

impl ManualTransactionTemplate {
    pub fn params_for_n_entries(n: u8) -> Vec<NewParamDefinition> {
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
        for i in 0..n {
            params.push(
                NewParamDefinition::builder()
                    .name(format!("entry_{}_account_id", i))
                    .r#type(ParamDataType::String)
                    .build()
                    .unwrap(),
            );
            params.push(
                NewParamDefinition::builder()
                    .name(format!("entry_{}_currency", i))
                    .r#type(ParamDataType::String)
                    .build()
                    .unwrap(),
            );
            params.push(
                NewParamDefinition::builder()
                    .name(format!("entry_{}_amount", i))
                    .r#type(ParamDataType::Decimal)
                    .build()
                    .unwrap(),
            );
            params.push(
                NewParamDefinition::builder()
                    .name(format!("entry_{}_description", i))
                    .r#type(ParamDataType::String)
                    .build()
                    .unwrap(),
            );
        }
        params
    }

    pub fn create_for_n_entries(n: u8) -> NewTxTemplate {
        unimplemented!()
    }
}
