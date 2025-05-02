use futures::StreamExt;
use lana_app::{app::LanaApp, primitives::*};
use lana_events::{CoreCreditEvent, LanaEvent};
use rust_decimal_macros::dec;
use tokio::task::JoinHandle;

use super::helpers;

pub async fn run(
    sub: &Subject,
    app: &LanaApp,
) -> anyhow::Result<Vec<JoinHandle<Result<(), anyhow::Error>>>> {
    let mut handles = Vec::new();
    let sub = *sub;

    {
        let app = app.clone();
        handles.push(tokio::spawn(async move {
            timely_payments_scenario(sub, &app).await
        }));
    }

    Ok(handles)
}

// Scenario 1: A credit facility that made timely payments and was paid off all according to the initial payment plan
async fn timely_payments_scenario(sub: Subject, app: &LanaApp) -> anyhow::Result<()> {
    let (customer_id, deposit_account_id) =
        helpers::create_customer(&sub, app, "1-timely-paid").await?;

    let deposit_amount = UsdCents::try_from_usd(dec!(10_000_000))?;
    helpers::make_deposit(&sub, app, &customer_id, deposit_amount).await?;

    let cf_terms = helpers::std_terms();
    let cf_amount = UsdCents::try_from_usd(dec!(10_000_000))?;
    let cf = app
        .credit()
        .initiate(&sub, customer_id, deposit_account_id, cf_amount, cf_terms)
        .await?;

    let mut stream = app.outbox().listen_persisted(None).await?;
    while let Some(msg) = stream.next().await {
        match &msg.payload {
            Some(LanaEvent::Credit(CoreCreditEvent::FacilityApproved { id })) if cf.id == *id => {
                app.credit()
                    .update_collateral(&sub, cf.id, Satoshis::try_from_btc(dec!(230))?)
                    .await?;
            }
            Some(LanaEvent::Credit(CoreCreditEvent::FacilityActivated { id, .. }))
                if cf.id == *id =>
            {
                app.credit()
                    .initiate_disbursal(&sub, cf.id, UsdCents::try_from_usd(dec!(1_000_000))?)
                    .await?;
            }
            Some(LanaEvent::Credit(CoreCreditEvent::ObligationDue {
                credit_facility_id: id,
                amount,
                ..
            })) if { cf.id == *id && amount > &UsdCents::ZERO } => {
                app.credit().record_payment(&sub, *id, *amount).await?;
                let facility = app
                    .credit()
                    .find_by_id(&sub, *id)
                    .await?
                    .expect("cf exists");
                if facility.interest_accrual_cycle_in_progress().is_none() {
                    dbg!("marker 3.1");
                    let total_outstanding_amount = app.credit().outstanding(&facility).await?;
                    dbg!(total_outstanding_amount);
                    app.credit()
                        .record_payment(&sub, facility.id, total_outstanding_amount)
                        .await?;
                    app.credit().complete_facility(&sub, facility.id).await?;
                }
            }
            Some(LanaEvent::Credit(CoreCreditEvent::FacilityCompleted { id, .. })) => {
                if cf.id == *id {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
