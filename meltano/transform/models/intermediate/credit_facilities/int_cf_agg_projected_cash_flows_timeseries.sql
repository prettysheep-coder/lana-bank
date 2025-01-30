WITH projected_cash_flows_common AS (
    SELECT
          *
    FROM {{ ref('int_cf_projected_cash_flows_common') }}
    WHERE credit_facility_end_date >= now_ts
), grouped AS (
    SELECT
          now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , SUM(projected_disbursal_amount_in_cents) AS projected_disbursal_amount_in_cents
        , days_from_now
        , SUM(projected_payment_amount_in_cents) AS projected_payment_amount_in_cents
    FROM projected_cash_flows_common
    GROUP BY
          now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , days_from_now
    ORDER BY days_from_now
)

SELECT
      *
    , TIMESTAMP(TIMESTAMP_ADD(DATE(now_ts), INTERVAL CAST(days_from_now AS INT64) DAY)) as date_from_now
    , SAFE_DIVIDE(projected_disbursal_amount_in_cents, 100.0) as projected_disbursal_amount_in_usd
    , SAFE_DIVIDE(projected_payment_amount_in_cents, 100.0) as projected_payment_amount_in_usd
FROM grouped
