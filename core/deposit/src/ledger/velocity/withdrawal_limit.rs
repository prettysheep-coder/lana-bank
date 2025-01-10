use rust_decimal::Decimal;
use tracing::instrument;

use cala_ledger::{
    tx_template::{Params, *},
    velocity::*,
    *,
};

use crate::ledger::error::*;

#[derive(Debug)]
pub struct WithdrawalLimitParams {
    pub withdrawal_limit: Decimal,
}

impl WithdrawalLimitParams {
    pub fn defs() -> Vec<NewParamDefinition> {
        vec![NewParamDefinition::builder()
            .name("withdrawal_limit")
            .r#type(ParamDataType::Decimal)
            .build()
            .unwrap()]
    }
}

impl From<WithdrawalLimitParams> for Params {
    fn from(params: WithdrawalLimitParams) -> Self {
        let mut p = Self::default();
        p.insert("withdrawal_limit", params.withdrawal_limit);
        p
    }
}

pub struct WithdrawalLimit;

impl WithdrawalLimit {
    #[instrument(name = "ledger.withdrawal_limit.create", skip_all)]
    pub async fn init(ledger: &CalaLedger) -> Result<VelocityLimitId, DepositLedgerError> {
        let limit = NewVelocityLimit::builder()
            .id(VelocityLimitId::new())
            .name("Withdrawal")
            .description("Defines the withdrawal limits")
            .window(vec![])
            .limit(
                NewLimit::builder()
                    .balance(vec![NewBalanceLimit::builder()
                        .layer("SETTLED")
                        .amount("params.withdrawal_limit")
                        .enforcement_direction("DEBIT")
                        .build()
                        .expect("balance limit")])
                    .build()
                    .expect("limit"),
            )
            .params(WithdrawalLimitParams::defs())
            .build()
            .expect("velocity limit");

        let created_limit = ledger.velocities().create_limit(limit).await?;

        Ok(created_limit.id())
    }
}
