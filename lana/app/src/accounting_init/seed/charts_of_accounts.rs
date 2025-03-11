use crate::{
    accounting_init::{constants::*, *},
    new_chart_of_accounts::NewChartOfAccounts,
};

use rbac_types::Subject;

pub(crate) async fn init(
    chart_of_accounts: &ChartOfAccounts,
    new_chart_of_accounts: &NewChartOfAccounts,
) -> Result<ChartsInit, AccountingInitError> {
    let chart_ids = &create_charts_of_accounts(chart_of_accounts).await?;
    create_new_chart_of_accounts(new_chart_of_accounts).await?;

    Ok(ChartsInit {
        chart_ids: *chart_ids,
    })
}

async fn create_new_chart_of_accounts(
    chart_of_accounts: &NewChartOfAccounts,
) -> Result<(), AccountingInitError> {
    if chart_of_accounts
        .find_by_reference(&Subject::System, CHART_REF.to_string())
        .await?
        .is_none()
    {
        chart_of_accounts
            .create_chart(
                &Subject::System,
                CHART_NAME.to_string(),
                CHART_REF.to_string(),
            )
            .await?;
    }

    Ok(())
}

async fn create_charts_of_accounts(
    chart_of_accounts: &ChartOfAccounts,
) -> Result<ChartIds, AccountingInitError> {
    let primary = match chart_of_accounts
        .find_by_reference(CHART_REF.to_string())
        .await?
    {
        Some(chart) => chart,
        None => {
            chart_of_accounts
                .create_chart(
                    ChartId::new(),
                    CHART_NAME.to_string(),
                    CHART_REF.to_string(),
                )
                .await?
        }
    };

    let off_balance_sheet = match chart_of_accounts
        .find_by_reference(OBS_CHART_REF.to_string())
        .await?
    {
        Some(chart) => chart,
        None => {
            chart_of_accounts
                .create_chart(
                    ChartId::new(),
                    OBS_CHART_NAME.to_string(),
                    OBS_CHART_REF.to_string(),
                )
                .await?
        }
    };

    Ok(ChartIds {
        primary: primary.id,
        off_balance_sheet: off_balance_sheet.id,
    })
}
