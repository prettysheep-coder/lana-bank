use async_graphql::{types::connection::*, *};
use serde::{Deserialize, Serialize};

use crate::{
    app::LavaApp,
    primitives::UsdCents,
    server::{
        admin::AdminAuthContext,
        shared_graphql::{customer::Customer, primitives::UUID},
    },
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Withdrawal {
    customer_id: UUID,
    withdrawal_id: UUID,
    amount: UsdCents,
}

#[ComplexObject]
impl Withdrawal {
    async fn customer(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Customer>> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let customer = app
            .customers()
            .find_by_id(Some(sub), &self.customer_id)
            .await?;
        Ok(customer.map(Customer::from))
    }
}

impl From<crate::withdraw::Withdraw> for Withdrawal {
    fn from(withdraw: crate::withdraw::Withdraw) -> Self {
        Withdrawal {
            withdrawal_id: UUID::from(withdraw.id),
            customer_id: UUID::from(withdraw.customer_id),
            amount: withdraw.amount,
        }
    }
}

#[derive(InputObject)]
pub struct WithdrawalInitiateInput {
    pub customer_id: UUID,
    pub amount: UsdCents,
    pub reference: Option<String>,
}

#[derive(SimpleObject)]
pub struct WithdrawalInitiatePayload {
    pub withdrawal: Withdrawal,
}

impl From<crate::withdraw::Withdraw> for WithdrawalInitiatePayload {
    fn from(withdrawal: crate::withdraw::Withdraw) -> Self {
        Self {
            withdrawal: Withdrawal::from(withdrawal),
        }
    }
}

#[derive(InputObject)]
pub struct WithdrawalConfirmInput {
    pub withdrawal_id: UUID,
}

#[derive(SimpleObject)]
pub struct WithdrawalConfirmPayload {
    pub withdrawal: Withdrawal,
}

impl From<crate::withdraw::Withdraw> for WithdrawalConfirmPayload {
    fn from(withdrawal: crate::withdraw::Withdraw) -> Self {
        Self {
            withdrawal: Withdrawal::from(withdrawal),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct WithdrawCursor {
    pub withdrawal_created_at: chrono::DateTime<chrono::Utc>,
}

impl CursorType for WithdrawCursor {
    type Error = String;

    fn encode_cursor(&self) -> String {
        use base64::{engine::general_purpose, Engine as _};
        let json = serde_json::to_string(&self).expect("could not serialize token");
        general_purpose::STANDARD_NO_PAD.encode(json.as_bytes())
    }

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        use base64::{engine::general_purpose, Engine as _};
        let bytes = general_purpose::STANDARD_NO_PAD
            .decode(s.as_bytes())
            .map_err(|e| e.to_string())?;
        let json = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for WithdrawCursor {
    fn from(withdrawal_created_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            withdrawal_created_at,
        }
    }
}

impl From<WithdrawCursor> for crate::withdraw::WithdrawCursor {
    fn from(cursor: WithdrawCursor) -> Self {
        Self {
            withdrawal_created_at: cursor.withdrawal_created_at,
        }
    }
}
