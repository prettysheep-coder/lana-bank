use serde::{Deserialize, Serialize};

use super::{DepositId, WithdrawalId};
use core_money::UsdCents;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CoreDepositEvent {
    DepositInitialized { id: DepositId, amount: UsdCents },
    WithdrawalConfirmed { id: WithdrawalId, amount: UsdCents },
}
