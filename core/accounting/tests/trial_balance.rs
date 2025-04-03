mod helpers;

use chrono::Utc;
use rand::Rng;

use authz::dummy::{DummyPerms, DummySubject};
use helpers::{action, object};

use core_accounting::{CalaJournalId, Chart, CoreAccounting, TrialBalances};

use cala_ledger::{CalaLedger, CalaLedgerConfig};

pub async fn init_chart(
    pool: &sqlx::Pool<sqlx::Postgres>,
    authz: &DummyPerms<action::DummyAction, object::DummyObject>,
    cala: &CalaLedger,
    journal_id: CalaJournalId,
    subject: &DummySubject,
) -> anyhow::Result<Chart> {
    let accounting = CoreAccounting::new(pool, authz, cala, journal_id);

    let rand_ref = format!("{:05}", rand::thread_rng().gen_range(0..100000));
    let chart_id = accounting
        .chart_of_accounts()
        .create_chart(subject, "Test Chart".to_string(), rand_ref.clone())
        .await?
        .id;

    let data = format!(
        r#"
        {rand_ref},,,Assets
        {rand_ref}1,,,Assets
        ,01,,Cash
        ,,0101,Central Office,
        ,02,,Payables
        ,,0101,Central Office,
        "#
    );

    Ok(accounting
        .chart_of_accounts()
        .import_from_csv(subject, chart_id, data)
        .await?)
}

#[tokio::test]
async fn add_chart_to_trial_balance() -> anyhow::Result<()> {
    let pool = helpers::init_pool().await?;
    let authz = authz::dummy::DummyPerms::<action::DummyAction, object::DummyObject>::new();

    let cala_config = CalaLedgerConfig::builder()
        .pool(pool.clone())
        .exec_migrations(false)
        .build()?;
    let cala = CalaLedger::init(cala_config).await?;

    let journal_id = helpers::init_journal(&cala).await?;

    let trial_balance_name = format!(
        "Trial Balance #{:05}",
        rand::thread_rng().gen_range(0..100000)
    );
    let trial_balances = TrialBalances::new(&pool, &authz, &cala, journal_id);
    trial_balances
        .create_trial_balance_statement(trial_balance_name.to_string())
        .await?;
    let trial_balance = trial_balances
        .trial_balance_accounts(
            &DummySubject,
            trial_balance_name.to_string(),
            Utc::now(),
            None,
            Default::default(),
        )
        .await?;
    assert_eq!(trial_balance.entities.len(), 0);

    let chart = init_chart(&pool, &authz, &cala, journal_id, &DummySubject).await?;
    trial_balances
        .add_chart_to_trial_balance(trial_balance_name.to_string(), chart)
        .await?;
    let trial_balance = trial_balances
        .trial_balance_accounts(
            &DummySubject,
            trial_balance_name.to_string(),
            Utc::now(),
            None,
            Default::default(),
        )
        .await?;
    assert_eq!(trial_balance.entities.len(), 2);

    Ok(())
}
