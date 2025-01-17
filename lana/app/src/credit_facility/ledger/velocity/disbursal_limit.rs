use rust_decimal::Decimal;
use tracing::instrument;

use cala_ledger::{velocity::*, *};

pub struct DisbursalLimit;

const DISBURSAL_LIMIT_ID: uuid::Uuid = uuid::uuid!("00000000-0000-0000-0000-000000000002");

#[derive(Debug)]
pub struct DisbursalLimitParams {
    pub disbursal_limit: Decimal,
}

impl DisbursalLimitParams {
    pub fn defs() -> Vec<NewParamDefinition> {
        vec![NewParamDefinition::builder()
            .name("disbursal_limit")
            .r#type(ParamDataType::Decimal)
            .build()
            .unwrap()]
    }
}

impl From<DisbursalLimitParams> for Params {
    fn from(params: DisbursalLimitParams) -> Self {
        let mut p = Self::default();
        p.insert("disbursal_limit", params.disbursal_limit);
        p
    }
}

impl DisbursalLimit {
    #[instrument(name = "ledger.overdraft_prevention.init", skip_all)]
    pub async fn init(
        ledger: &CalaLedger,
    ) -> Result<VelocityLimitId, crate::credit_facility::ledger::CreditLedgerError> {
        let params = DisbursalLimitParams::defs();

        let limit = NewVelocityLimit::builder()
            .id(DISBURSAL_LIMIT_ID)
            .name("Disbursal Limit")
            .description("Limit for disbursals")
            .window(vec![])
            .limit(
                NewLimit::builder()
                    .balance(vec![NewBalanceLimit::builder()
                        .layer("SETTLED")
                        .amount("params.disbursal_limit")
                        .enforcement_direction("DEBIT")
                        .build()
                        .expect("balance limit")])
                    .build()
                    .expect("limit"),
            )
            .params(params)
            .build()
            .expect("velocity limit");

        match ledger.velocities().create_limit(limit).await {
            Err(cala_ledger::velocity::error::VelocityError::LimitIdAlreadyExists) => {
                Ok(DISBURSAL_LIMIT_ID.into())
            }
            Err(e) => Err(e.into()),
            Ok(limit) => Ok(limit.id()),
        }
    }
}
