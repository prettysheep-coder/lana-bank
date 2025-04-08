mod helpers;

use authz::dummy::DummySubject;
use cala_ledger::{CalaLedger, CalaLedgerConfig, Currency, DebitOrCredit};
use core_accounting::{CoreAccounting, ManualEntryInput};
use helpers::{action, object};
use rust_decimal_macros::dec;

#[tokio::test]
#[rustfmt::skip]
async fn manual_transaction_with_two_entries() -> anyhow::Result<()> {
    use rand::Rng;
    let pool = helpers::init_pool().await?;
    let cala_config = CalaLedgerConfig::builder().pool(pool.clone()).exec_migrations(false).build()?;
    let cala = CalaLedger::init(cala_config).await?;
    let authz = authz::dummy::DummyPerms::<action::DummyAction, object::DummyObject>::new();
    let journal_id = helpers::init_journal(&cala).await?;

    let accounting = CoreAccounting::new(&pool, &authz, &cala, journal_id);
    let chart_ref = format!("ref-{:08}", rand::thread_rng().gen_range(0..10000));
    let chart = accounting.chart_of_accounts().create_chart(&DummySubject, "Test chart".to_string(), chart_ref.clone()).await?;
    let import = r#"
        1,,Asset
        2,,Liabilities
        "#;
    let chart_id = chart.id;
    let _ = accounting.chart_of_accounts().import_from_csv(&DummySubject, chart_id, import).await?;

    let entries = vec![
        ManualEntryInput::builder().account_id_or_code("1".parse().unwrap()).amount(dec!(100)).currency(Currency::USD).direction(DebitOrCredit::Debit).description("test debit").build().unwrap(),
        ManualEntryInput::builder().account_id_or_code("2".parse().unwrap()).amount(dec!(100)).currency(Currency::USD).direction(DebitOrCredit::Credit).description("test credit").build().unwrap(),
    ];
    accounting.execute_manual_transaction(&DummySubject, &chart_ref, None, "Test transaction".to_string(), entries).await?;
    Ok(())
}
