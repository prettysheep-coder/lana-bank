use crate::primitives::BfxAddressType;

use super::cala::graphql::*;

impl From<BfxAddressType> for bfx_address_backed_account_create::BfxAddressType {
    fn from(address_type: BfxAddressType) -> Self {
        match address_type {
            BfxAddressType::Bitcoin => bfx_address_backed_account_create::BfxAddressType::BTC,
            BfxAddressType::Tron => bfx_address_backed_account_create::BfxAddressType::TRX,
        }
    }
}
