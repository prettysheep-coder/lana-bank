#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod config;

use futures::StreamExt;
use rust_decimal_macros::dec;

use std::collections::HashSet;

use lana_app::{
    app::LanaApp,
    primitives::{CreditFacilityId, Satoshis, Subject, UsdCents},
    terms::{Duration, InterestInterval, TermValues},
};
use lana_events::*;

pub use config::*;

const CUSTOMER_EMAIL: &'static str = "bootstrap-lana@galoy.io";
const CUSTOMER_TELEGRAM: &'static str = "bootstrap-lana";

pub async fn run(
    superuser_email: String,
    app: &LanaApp,
    config: BootstrapConfig,
) -> anyhow::Result<()> {
    dbg!(&config);
    let sub = superuser_subject(&superuser_email, app).await?;
    initial_setup(&sub, app, &config).await?;

    let mut facility_ids = HashSet::new();
    for _ in 0..config.num_facilities {
        let id = bootstrap_credit_facility(&sub, app).await?;
        facility_ids.insert(id);
    }

    let spawned_app = app.clone();
    let _handle = tokio::spawn(async move {
        let _ = process_repayment(sub, facility_ids, spawned_app).await;
    });

    println!("waiting for real time");
    sim_time::wait_until_realtime().await;
    println!("done");

    Ok(())
}

async fn process_repayment(
    sub: Subject,
    facility_ids: HashSet<CreditFacilityId>,
    app: LanaApp,
) -> anyhow::Result<()> {
    let mut stream = app.outbox().listen_persisted(None).await?;

    while let Some(msg) = stream.next().await {
        match &msg.payload {
            Some(LanaEvent::Credit(CreditEvent::AccrualExecuted {
                id: cf_id, amount, ..
            })) if facility_ids.contains(cf_id) && amount > &UsdCents::ZERO => {
                let _ = app
                    .credit_facilities()
                    .record_payment(&sub, *cf_id, *amount)
                    .await;
                let mut cf = app
                    .credit_facilities()
                    .find_by_id(&sub, *cf_id)
                    .await?
                    .expect("cf exists");
                if cf.interest_accrual_in_progress().is_none() {
                    app.credit_facilities()
                        .record_payment(&sub, cf.id, cf.outstanding().total())
                        .await?;
                    app.credit_facilities()
                        .complete_facility(&sub, cf.id)
                        .await?;
                }
            }
            Some(LanaEvent::Credit(CreditEvent::FacilityCompleted { id: cf_id, .. }))
                if facility_ids.contains(cf_id) =>
            {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn initial_setup(
    sub: &Subject,
    app: &LanaApp,
    config: &BootstrapConfig,
) -> anyhow::Result<()> {
    if app
        .customers()
        .find_by_email(sub, CUSTOMER_EMAIL.to_string())
        .await?
        .is_none()
    {
        let customer = app
            .customers()
            .create(
                sub,
                CUSTOMER_EMAIL.to_string(),
                CUSTOMER_TELEGRAM.to_string(),
            )
            .await?;

        let deposit_account_id = app
            .deposits()
            .list_account_by_created_at_for_account_holder(
                sub,
                customer.id,
                Default::default(),
                es_entity::ListDirection::Descending,
            )
            .await?
            .entities
            .into_iter()
            .next()
            .expect("Deposit account not found")
            .id;

        let _ = app
            .deposits()
            .record_deposit(
                sub,
                deposit_account_id,
                UsdCents::try_from_usd(
                    rust_decimal::Decimal::from(config.num_facilities) * dec!(1_000_000),
                )?,
                None,
            )
            .await?;
    }

    Ok(())
}

async fn superuser_subject(superuser_email: &String, app: &LanaApp) -> anyhow::Result<Subject> {
    let superuser = app
        .users()
        .find_by_email(None, superuser_email)
        .await?
        .expect("Superuser not found");
    Ok(Subject::from(superuser.id))
}

async fn bootstrap_credit_facility(
    sub: &Subject,
    app: &LanaApp,
) -> anyhow::Result<CreditFacilityId> {
    let customer = app
        .customers()
        .find_by_email(sub, CUSTOMER_EMAIL.to_string())
        .await?
        .expect("customer not found");

    let terms = std_terms();

    let cf = app
        .credit_facilities()
        .initiate(
            sub,
            customer.id,
            UsdCents::try_from_usd(dec!(10_000_000))?,
            terms,
        )
        .await?;

    let id = cf.id;

    // hack to ensure that credit facility is activated
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    app.credit_facilities()
        .update_collateral(sub, id, Satoshis::try_from_btc(dec!(230))?)
        .await?;

    // hack to ensure that credit facility is activated
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    app.credit_facilities()
        .initiate_disbursal(sub, id, UsdCents::try_from_usd(dec!(1_000_000))?)
        .await?;
    Ok(id)
}

fn std_terms() -> TermValues {
    TermValues::builder()
        .annual_rate(dec!(12))
        .initial_cvl(dec!(140))
        .margin_call_cvl(dec!(125))
        .liquidation_cvl(dec!(105))
        .duration(Duration::Months(3))
        .incurrence_interval(InterestInterval::EndOfDay)
        .accrual_interval(InterestInterval::EndOfMonth)
        .one_time_fee_rate(dec!(0.01))
        .build()
        .unwrap()
}
