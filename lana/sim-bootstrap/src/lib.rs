#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

mod config;

use futures::StreamExt;
use rust_decimal_macros::dec;
use std::sync::{Arc, Mutex};

use std::collections::HashSet;

use lana_app::{
    app::LanaApp,
    primitives::*,
    terms::{Duration, InterestInterval, TermValues},
};
use lana_events::*;

pub use config::*;

pub async fn run(
    superuser_email: String,
    app: LanaApp,
    config: BootstrapConfig,
) -> anyhow::Result<()> {
    dbg!(&config);
    let sub = superuser_subject(&superuser_email, &app).await?;

    let customer_ids = create_customers(&sub, &app, &config).await?;

    make_deposit(&sub, &app, &customer_ids, &config).await?;

    let mut facility_ids = HashSet::new();

    for customer_id in customer_ids {
        for _ in 0..config.num_facilities {
            let id = create_facility_for_customer(&sub, &app, customer_id).await?;
            facility_ids.insert(id);
        }
    }

    let facility_ids = Arc::new(Mutex::new(facility_ids));

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
    facility_ids: Arc<Mutex<HashSet<CreditFacilityId>>>,
    app: LanaApp,
) -> anyhow::Result<()> {
    let mut stream = app.outbox().listen_persisted(None).await?;

    while let Some(msg) = stream.next().await {
        match &msg.payload {
            Some(LanaEvent::Credit(CreditEvent::AccrualExecuted {
                id: cf_id, amount, ..
            })) if {
                let ids = facility_ids.lock().unwrap();
                ids.contains(cf_id) && amount > &UsdCents::ZERO
            } =>
            {
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
            Some(LanaEvent::Credit(CreditEvent::FacilityCompleted { id: cf_id, .. })) => {
                let mut ids = facility_ids.lock().unwrap();
                if ids.remove(cf_id) && ids.is_empty() {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn create_customers(
    sub: &Subject,
    app: &LanaApp,
    config: &BootstrapConfig,
) -> anyhow::Result<Vec<CustomerId>> {
    let mut customer_ids = Vec::new();

    for i in 1..=config.num_customers {
        let customer_email = format!("customer{}@example.com", i);
        let telegram = format!("customer{}", i);

        let customer = match app
            .customers()
            .find_by_email(sub, customer_email.clone())
            .await?
        {
            Some(existing_customer) => existing_customer,
            None => {
                app.customers()
                    .create(sub, customer_email.clone(), telegram)
                    .await?
            }
        };

        customer_ids.push(customer.id);
    }

    Ok(customer_ids)
}

async fn make_deposit(
    sub: &Subject,
    app: &LanaApp,
    customer_ids: &Vec<CustomerId>,
    config: &BootstrapConfig,
) -> anyhow::Result<()> {
    for customer_id in customer_ids {
        let deposit_account_id = app
            .deposits()
            .list_account_by_created_at_for_account_holder(
                sub,
                *customer_id,
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

async fn create_facility_for_customer(
    sub: &Subject,
    app: &LanaApp,
    customer_id: CustomerId,
) -> anyhow::Result<CreditFacilityId> {
    let terms = std_terms();

    let cf = app
        .credit_facilities()
        .initiate(
            sub,
            customer_id,
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
