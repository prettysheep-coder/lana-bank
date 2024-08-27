mod bfx_client;
mod error;

use crate::primitives::{PriceOfOneBTC, UsdCents};

use bfx_client::BfxClient;
use error::PriceError;

pub struct Price {
    bfx: BfxClient,
}

impl Price {
    pub fn init() -> Result<Self, PriceError> {
        Ok(Price {
            bfx: BfxClient::init()?,
        })
    }

    pub async fn usd_cents_per_btc(&self) -> Result<PriceOfOneBTC, PriceError> {
        let last_price = self.bfx.btc_usd_tick().await?.last_price;
        Ok(PriceOfOneBTC::new(UsdCents::try_from_usd(last_price)?))
    }
}
