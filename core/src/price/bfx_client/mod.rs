mod bfx_response;
pub mod error;

use reqwest::Client as ReqwestClient;

use bfx_response::{BfxErrorResponse, BtcUsdTick};
use error::BfxClientError;

const BASE_URL: &str = "https://api-pub.bitfinex.com/v2/";

#[derive(Clone)]
pub struct BfxClient {
    client: ReqwestClient,
}

impl BfxClient {
    pub fn init() -> Result<Self, BfxClientError> {
        Ok(BfxClient {
            client: ReqwestClient::builder().use_rustls_tls().build()?,
        })
    }

    pub async fn btc_usd_tick(&self) -> Result<BtcUsdTick, BfxClientError> {
        let url = format!("{}ticker/tBTCUSD", BASE_URL);
        let response = self
            .client
            .get(&url)
            .header("accept", "application/json")
            .send()
            .await?;
        let tick = Self::extract_response_data::<BtcUsdTick>(response).await?;

        Ok(tick)
    }

    async fn extract_response_data<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, BfxClientError> {
        let status = response.status();
        let response_text = response.text().await?;
        if status.is_success() {
            Ok(serde_json::from_str::<T>(&response_text)?)
        } else {
            let data = serde_json::from_str::<BfxErrorResponse>(&response_text)?;
            Err(BfxClientError::from((
                data.event,
                data.code,
                data.description,
            )))
        }
    }
}
