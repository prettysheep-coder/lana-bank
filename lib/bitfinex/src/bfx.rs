use core::fmt;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::config::Config;

use hmac::{Hmac, Mac};
use sha2::Sha384;

type HmacSha384 = Hmac<Sha384>;

use hex::encode;

const BITFINEX_BASE_URL: &str = "https://api.bitfinex.com";

fn get_nonce() -> Result<String, Box<dyn Error>> {
    Ok((SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() * 1000000).to_string())
}

struct SignOutput {
    signature: String,
    body_json: String,
}

fn sign<T>(
    body: &T,
    api_path: &str,
    nonce: &str,
    config: &Config,
) -> Result<SignOutput, Box<dyn Error>>
where
    T: Serialize,
{
    let body_json = serde_json::to_string(&body)?;

    let signature_payload = format!("/api/{}{}{}", api_path, nonce, body_json);

    let mut mac = HmacSha384::new_from_slice(&config.secret.as_bytes())?;
    mac.update(signature_payload.as_bytes());
    let signature = encode(mac.finalize().into_bytes());

    Ok(SignOutput {
        signature,
        body_json,
    })
}

#[derive(Serialize)]
struct BodyDepositAddressList {
    method: String,
}

pub async fn get_addresses(config: &Config) -> Result<(), Box<dyn Error>> {
    // let url = "https://api-pub.bitfinex.com/v2/funding/stats/fUSD/hist";

    // address list
    let api_path = "v2/auth/r/deposit/address/all";

    let nonce = get_nonce()?;

    let body: BodyDepositAddressList = BodyDepositAddressList {
        method: String::from("bitcoin"),
    };

    let sign_output = sign(&body, api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    println!("{}", json);

    Ok(())
}

#[derive(Serialize)]
struct BodyWalletList {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WalletInput {
    wallet_type: String,
    currency: String,
    balance: Decimal,
    unsettled_interest: Decimal,
    balance_available: Decimal,
    _unused1: Option<String>,
    _unused2: Option<String>,
}

type WalletInputList = Vec<WalletInput>;

#[derive(Debug)]
struct Wallet {
    wallet_type: String,
    currency: String,
    balance: Decimal,
    unsettled_interest: Decimal,
    balance_available: Decimal,
}

type WalletList = Vec<Wallet>;

pub async fn get_balances(config: &Config, currency: Option<String>) -> Result<(), Box<dyn Error>> {
    let api_path = "v2/auth/r/wallets";

    let nonce = get_nonce()?;

    let body: BodyWalletList = BodyWalletList {};

    let sign_output = sign(&body, api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let wallet_list: WalletInputList = serde_json::from_value(json)?;

    let wallet_list: WalletList = match currency {
        Some(currency) => wallet_list
            .into_iter()
            .filter(|w| w.currency == currency)
            .map(|w| Wallet {
                wallet_type: w.wallet_type,
                currency: w.currency,
                balance: w.balance,
                unsettled_interest: w.unsettled_interest,
                balance_available: w.balance_available,
            })
            .collect(),
        None => wallet_list
            .into_iter()
            .map(|w| Wallet {
                wallet_type: w.wallet_type,
                currency: w.currency,
                balance: w.balance,
                unsettled_interest: w.unsettled_interest,
                balance_available: w.balance_available,
            })
            .collect(),
    };

    println!("{:?}", wallet_list);

    Ok(())
}

#[derive(Serialize)]
struct BodyDepositAddress {
    wallet: String,
    method: String,
    op_renew: i32,
}

pub async fn create_bitcoin_address(config: &Config) -> Result<(), Box<dyn Error>> {
    let api_path = "v2/auth/w/deposit/address";

    let nonce = get_nonce()?;

    let body = BodyDepositAddress {
        wallet: "trading".to_string(),
        method: "bitcoin".to_string(),
        op_renew: 1,
    };

    let sign_output = sign(&body, api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    println!("{}", json);

    Ok(())
}

pub async fn create_trx_address(config: &Config) -> Result<(), Box<dyn Error>> {
    let api_path = "v2/auth/w/deposit/address";

    let nonce = get_nonce()?;

    let body = BodyDepositAddress {
        wallet: "trading".to_string(),
        method: "tetherusx".to_string(),
        // 0 = use previously generated
        // 1 = generate new
        op_renew: 0,
    };

    let sign_output = sign(&body, api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    println!("{}", json);

    Ok(())
}

#[derive(Serialize)]
struct BodyWithdrawUsdtTrx {
    wallet: String,
    method: String,
    amount: Decimal,
    address: String,
    fee_deduct: u8,
}

enum WalletType {
    Exchange,
    Margin,
    Funding,
}

impl fmt::Display for WalletType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            WalletType::Exchange => "exchange",
            WalletType::Margin => "margin",
            WalletType::Funding => "funding",
        };
        write!(f, "{}", s)
    }
}

pub async fn withdraw_usdt_trx(
    config: &Config,
    amount: Decimal,
    address: String,
) -> Result<(), Box<dyn Error>> {
    let api_path: &str = "v2/auth/w/withdraw";

    let nonce = get_nonce()?;

    let body = BodyWithdrawUsdtTrx {
        wallet: WalletType::Exchange.to_string(),
        method: "tetherusx".to_string(),
        amount,
        address,
        fee_deduct: 0,
    };

    let sign_output = sign(&body, api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    println!("{}", json);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct MovementInput {
    #[serde(rename = "0")]
    id: u64,
    #[serde(rename = "1")]
    currency: String,
    #[serde(rename = "2")]
    currency_name: String,
    #[serde(rename = "3")]
    _dummy1: Option<String>,
    #[serde(rename = "4")]
    _dummy2: Option<String>,
    #[serde(rename = "5")]
    mts_started: Option<i64>,
    #[serde(rename = "6")]
    mts_updated: Option<i64>,
    #[serde(rename = "7")]
    _dummy3: Option<String>,
    #[serde(rename = "8")]
    _dummy4: Option<String>,
    #[serde(rename = "9")]
    status: String,
    #[serde(rename = "10")]
    _dummy5: Option<String>,
    #[serde(rename = "11")]
    _dummy6: Option<String>,
    #[serde(rename = "12")]
    amount: Option<f64>,
    #[serde(rename = "13")]
    fees: Option<f64>,
    #[serde(rename = "14")]
    _dummy7: Option<String>,
    #[serde(rename = "15")]
    _dummy8: Option<String>,
    #[serde(rename = "16")]
    destination_address: Option<String>,
    #[serde(rename = "17")]
    payment_id: Option<String>,
    #[serde(rename = "18")]
    _dummy9: Option<String>,
    #[serde(rename = "19")]
    _dummy10: Option<String>,
    #[serde(rename = "20")]
    transaction_id: Option<String>,
    #[serde(rename = "21")]
    withdraw_transaction_note: Option<String>,
}

type MovementInputList = Vec<MovementInput>;

#[derive(Debug)]
struct WithdrawalTransaction {
    id: u64,
    currency: String,
    currency_name: String,
    mts_started: Option<NaiveDateTime>,
    mts_updated: Option<NaiveDateTime>,
    status: String,
    amount: Option<f64>,
    fees: Option<f64>,
    send_to_address: String,
    payment_id: Option<String>,
    transaction_id: Option<String>,
    withdraw_transaction_note: Option<String>,
}

#[derive(Debug)]
struct DepositTransaction {
    id: u64,
    currency: String,
    currency_name: String,
    mts_started: Option<NaiveDateTime>,
    mts_updated: Option<NaiveDateTime>,
    status: String,
    amount: Option<f64>,
    fees: Option<f64>,
    receive_from_address: String,
    payment_id: Option<String>,
    transaction_id: Option<String>,
    withdraw_transaction_note: Option<String>,
}

#[derive(Debug)]
enum Transaction {
    Withdrawal(WithdrawalTransaction),
    Deposit(DepositTransaction),
}

type MovementList = Vec<Transaction>;

#[derive(Serialize)]
struct BodyTransactionsList {}

use chrono::{DateTime, NaiveDateTime, Utc};

fn i64_to_datetime_opt(timestamp: Option<i64>) -> Option<NaiveDateTime> {
    let res =
        timestamp.map(|ts| DateTime::from_timestamp(ts / 1000, (ts % 1000 * 1_000_000) as u32));
    match res {
        Some(res) => Some(res.unwrap().naive_utc()),
        None => None,
    }
}

pub async fn get_transactions(
    config: &Config,
    currency: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let api_path = match currency {
        Some(currency) => format!("v2/auth/r/movements/{currency}/hist"),
        None => "v2/auth/r/movements/hist".to_string(),
    };

    let nonce = get_nonce()?;

    let body: BodyTransactionsList = BodyTransactionsList {};

    let sign_output = sign(&body, &api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await
        .expect("Could not send request");

    let json: MovementInputList = res.json().await.expect("could not decode request");
    let json: MovementList = json
        .into_iter()
        .map(|m| {
            if m.amount.unwrap() > 0.0 {
                Transaction::Deposit(DepositTransaction {
                    id: m.id,
                    currency: m.currency,
                    currency_name: m.currency_name,
                    mts_started: i64_to_datetime_opt(m.mts_started),
                    mts_updated: i64_to_datetime_opt(m.mts_updated),
                    status: m.status,
                    amount: m.amount,
                    fees: m.fees,
                    receive_from_address: m.destination_address.unwrap(),
                    payment_id: m.payment_id,
                    transaction_id: m.transaction_id,
                    withdraw_transaction_note: m.withdraw_transaction_note,
                })
            } else {
                Transaction::Withdrawal(WithdrawalTransaction {
                    id: m.id,
                    currency: m.currency,
                    currency_name: m.currency_name,
                    mts_started: i64_to_datetime_opt(m.mts_started),
                    mts_updated: i64_to_datetime_opt(m.mts_updated),
                    status: m.status,
                    amount: m.amount,
                    fees: m.fees,
                    send_to_address: m.destination_address.unwrap(),
                    payment_id: m.payment_id,
                    transaction_id: m.transaction_id,
                    withdraw_transaction_note: m.withdraw_transaction_note,
                })
            }
        })
        .collect();

    // let mut set_of_currency = HashSet::new();
    // json.iter().for_each(|m| {
    //     set_of_currency.insert(m.currency.clone());
    // });
    // println!("{:?}", set_of_currency);

    println!("{:?}", json);

    Ok(())
}

use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawUserInfo {
    #[serde(rename = "0")]
    id: u64,
    #[serde(rename = "1")]
    email: String,
    #[serde(rename = "2")]
    username: String,
    #[serde(rename = "3")]
    mts_account_create: u64,
    #[serde(rename = "4")]
    verified: u8,
    #[serde(rename = "5")]
    verification_level: u8,
    #[serde(rename = "6")]
    _dummy1: Option<Value>,
    #[serde(rename = "7")]
    timezone: Option<String>,
    #[serde(rename = "8")]
    locale: Option<String>,
    #[serde(rename = "9")]
    company: Option<String>,
    #[serde(rename = "10")]
    email_verified: u8,
    #[serde(rename = "11")]
    _dummy2: Option<Value>,
    #[serde(rename = "12")]
    _dummy3: Option<Value>,
    #[serde(rename = "13")]
    _dummy4: Option<Value>,
    #[serde(rename = "14")]
    mts_master_account_create: Option<u64>,
    #[serde(rename = "15")]
    group_id: Option<u64>,
    #[serde(rename = "16")]
    master_account_id: Option<u64>,
    #[serde(rename = "17")]
    inherit_master_account_verification: Option<u8>,
    #[serde(rename = "18")]
    is_group_master: Option<u8>,
    #[serde(rename = "19")]
    group_withdraw_enabled: Option<u8>,
    #[serde(rename = "20")]
    _dummy5: Option<Value>,
    #[serde(rename = "21")]
    ppt_enabled: Option<u8>,
    #[serde(rename = "22")]
    merchant_enabled: Option<u8>,
    #[serde(rename = "23")]
    competition_enabled: Option<u8>,
    #[serde(rename = "24")]
    _dummy6: Option<Value>,
    #[serde(rename = "25")]
    _dummy7: Option<Value>,
    #[serde(rename = "26")]
    two_fa_modes: Option<Vec<String>>,
    #[serde(rename = "27")]
    _dummy8: Option<Value>,
    #[serde(rename = "28")]
    is_securities_master: Option<u8>,
    #[serde(rename = "29")]
    securities_enabled: Option<u8>,
    #[serde(rename = "30")]
    is_securities_investor_accredited: Option<u8>,
    #[serde(rename = "31")]
    is_securities_el_salvador: Option<u8>,
    #[serde(rename = "32")]
    _dummy9: Option<Value>,
    #[serde(rename = "33")]
    _dummy10: Option<u8>,
    #[serde(rename = "34")]
    _dummy11: Option<Value>,
    #[serde(rename = "35")]
    _dummy12: Option<Value>,
    #[serde(rename = "36")]
    _dummy13: Option<Value>,
    #[serde(rename = "37")]
    _dummy14: Option<Value>,
    #[serde(rename = "38")]
    allow_disable_ctxswitch: Option<u8>,
    #[serde(rename = "39")]
    ctxswitch_disabled: Option<u8>,
    #[serde(rename = "40")]
    _dummy15: Option<Value>,
    #[serde(rename = "41")]
    _dummy16: Option<Value>,
    #[serde(rename = "42")]
    _dummy17: Option<Value>,
    #[serde(rename = "43")]
    _dummy18: Option<Value>,
    #[serde(rename = "44")]
    time_last_login: Option<String>,
    #[serde(rename = "45")]
    _dummy19: Option<Value>,
    #[serde(rename = "46")]
    _dummy20: Option<Value>,
    #[serde(rename = "47")]
    verification_level_submitted: Option<u8>,
    #[serde(rename = "48")]
    _dummy21: Option<Value>,
    #[serde(rename = "49")]
    comp_countries: Option<Vec<String>>,
    #[serde(rename = "50")]
    comp_countries_resid: Option<Vec<String>>,
    #[serde(rename = "51")]
    compl_account_type: Option<String>,
    #[serde(rename = "52")]
    _dummy22: Option<Value>,
    #[serde(rename = "53")]
    _dummy23: Option<Value>,
    #[serde(rename = "54")]
    is_merchant_enterprise: Option<u8>,
}

#[derive(Debug)]
struct UserInfo {
    id: u64,
    email: String,
    username: String,
    mts_account_create: DateTime<Utc>,
    verified: u8,
    verification_level: u8,
    timezone: Option<String>,
    locale: Option<String>,
    company: Option<String>,
    email_verified: u8,
    mts_master_account_create: Option<u64>,
    group_id: Option<u64>,
    master_account_id: Option<u64>,
    inherit_master_account_verification: Option<u8>,
    is_group_master: Option<u8>,
    group_withdraw_enabled: Option<u8>,
    ppt_enabled: Option<u8>,
    merchant_enabled: Option<u8>,
    competition_enabled: Option<u8>,
    two_fa_modes: Option<Vec<String>>,
    is_securities_master: Option<u8>,
    securities_enabled: Option<u8>,
    is_securities_investor_accredited: Option<u8>,
    is_securities_el_salvador: Option<u8>,
    allow_disable_ctxswitch: Option<u8>,
    ctxswitch_disabled: Option<u8>,
    time_last_login: Option<String>,
    verification_level_submitted: Option<u8>,
    comp_countries: Option<Vec<String>>,
    comp_countries_resid: Option<Vec<String>>,
    compl_account_type: Option<String>,
    is_merchant_enterprise: Option<u8>,
}

fn timestamp_to_datetime(timestamp: u64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp((timestamp / 1000) as i64, 0);
    DateTime::from_utc(naive, Utc)
}

pub async fn get_user_info(config: &Config) -> Result<(), Box<dyn Error>> {
    let api_path = "v2/auth/r/info/user";

    let nonce = get_nonce()?;

    let body: BodyTransactionsList = BodyTransactionsList {};

    let sign_output = sign(&body, &api_path, &nonce, config)?;
    let SignOutput {
        signature,
        body_json,
    } = sign_output;

    let client = Client::new();
    let res = client
        .post(format!("{BITFINEX_BASE_URL}/{}", api_path))
        .header("Content-Type", "application/json")
        .header("bfx-nonce", &nonce)
        .header("bfx-apikey", &config.key)
        .header("bfx-signature", signature)
        .body(body_json)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let raw_user_info: RawUserInfo = serde_json::from_value(json)?;

    let user_info = UserInfo {
        id: raw_user_info.id,
        email: raw_user_info.email,
        username: raw_user_info.username,
        mts_account_create: timestamp_to_datetime(raw_user_info.mts_account_create),
        verified: raw_user_info.verified,
        verification_level: raw_user_info.verification_level,
        timezone: raw_user_info.timezone,
        locale: raw_user_info.locale,
        company: raw_user_info.company,
        email_verified: raw_user_info.email_verified,
        mts_master_account_create: raw_user_info.mts_master_account_create,
        group_id: raw_user_info.group_id,
        master_account_id: raw_user_info.master_account_id,
        inherit_master_account_verification: raw_user_info.inherit_master_account_verification,
        is_group_master: raw_user_info.is_group_master,
        group_withdraw_enabled: raw_user_info.group_withdraw_enabled,
        ppt_enabled: raw_user_info.ppt_enabled,
        merchant_enabled: raw_user_info.merchant_enabled,
        competition_enabled: raw_user_info.competition_enabled,
        two_fa_modes: raw_user_info.two_fa_modes,
        is_securities_master: raw_user_info.is_securities_master,
        securities_enabled: raw_user_info.securities_enabled,
        is_securities_investor_accredited: raw_user_info.is_securities_investor_accredited,
        is_securities_el_salvador: raw_user_info.is_securities_el_salvador,
        allow_disable_ctxswitch: raw_user_info.allow_disable_ctxswitch,
        ctxswitch_disabled: raw_user_info.ctxswitch_disabled,
        time_last_login: raw_user_info.time_last_login,
        verification_level_submitted: raw_user_info.verification_level_submitted,
        comp_countries: raw_user_info.comp_countries,
        comp_countries_resid: raw_user_info.comp_countries_resid,
        compl_account_type: raw_user_info.compl_account_type,
        is_merchant_enterprise: raw_user_info.is_merchant_enterprise,
    };

    println!("{:?}", user_info);

    Ok(())
}
