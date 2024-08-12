use async_graphql::{types::connection::*, *};
use serde::{Deserialize, Serialize};

use crate::{
    app::LavaApp,
    primitives::UsdCents,
    server::{
        admin::AdminAuthContext,
        shared_graphql::{customer::Customer, primitives::*},
    },
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Deposit {
    customer_id: UUID,
    deposit_id: UUID,
    amount: UsdCents,
}

#[ComplexObject]
impl Deposit {
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

impl From<crate::deposit::Deposit> for Deposit {
    fn from(deposit: crate::deposit::Deposit) -> Self {
        Deposit {
            deposit_id: UUID::from(deposit.id),
            customer_id: UUID::from(deposit.customer_id),
            amount: deposit.amount,
        }
    }
}
#[derive(InputObject)]
pub struct DepositRecordInput {
    pub customer_id: UUID,
    pub amount: UsdCents,
    pub reference: Option<String>,
}

#[derive(SimpleObject)]
pub struct DepositRecordPayload {
    pub deposit: Deposit,
}

impl From<crate::deposit::Deposit> for DepositRecordPayload {
    fn from(deposit: crate::deposit::Deposit) -> Self {
        Self {
            deposit: Deposit::from(deposit),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct DepositCursor {
    pub deposit_created_at: chrono::DateTime<chrono::Utc>,
}

impl CursorType for DepositCursor {
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

impl From<chrono::DateTime<chrono::Utc>> for DepositCursor {
    fn from(deposit_created_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self { deposit_created_at }
    }
}

impl From<DepositCursor> for crate::deposit::DepositCursor {
    fn from(cursor: DepositCursor) -> Self {
        Self {
            deposit_created_at: cursor.deposit_created_at,
        }
    }
}
