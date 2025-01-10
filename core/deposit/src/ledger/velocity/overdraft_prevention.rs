use tracing::instrument;

use cala_ledger::{velocity::*, *};

use crate::ledger::error::*;

pub struct OverdraftPrevention;

impl OverdraftPrevention {
    #[instrument(name = "ledger.overdraft_prevention.init", skip_all)]
    pub async fn init(ledger: &CalaLedger) -> Result<VelocityLimitId, DepositLedgerError> {
        let limit = NewVelocityLimit::builder()
            .id(VelocityLimitId::new())
            .name("Overdraft Prevention")
            .description("Prevent overdraft on withdrawals")
            .window(vec![])
            .limit(
                NewLimit::builder()
                    .balance(vec![NewBalanceLimit::builder()
                        .layer("SETTLED")
                        .amount("decimal('0.0')")
                        .enforcement_direction("DEBIT")
                        .build()
                        .expect("balance limit")])
                    .build()
                    .expect("limit"),
            )
            .build()
            .expect("velocity limit");

        let created_limit = ledger.velocities().create_limit(limit).await?;

        Ok(created_limit.id())
    }
}
