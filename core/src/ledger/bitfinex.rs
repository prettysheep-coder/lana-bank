use crate::primitives::{BfxAddressType, BfxIntegrationId};

use super::cala::graphql::*;

impl From<BfxAddressType> for bfx_address_backed_account_create::BfxAddressType {
    fn from(address_type: BfxAddressType) -> Self {
        match address_type {
            BfxAddressType::Bitcoin => bfx_address_backed_account_create::BfxAddressType::BTC,
            BfxAddressType::Tron => bfx_address_backed_account_create::BfxAddressType::TRX,
        }
    }
}

impl From<bfx_integration_by_id::BfxIntegrationByIdBitfinexIntegration> for BfxIntegrationId {
    fn from(bfx_integration: bfx_integration_by_id::BfxIntegrationByIdBitfinexIntegration) -> Self {
        Self::from(bfx_integration.integration_id)
    }
}
