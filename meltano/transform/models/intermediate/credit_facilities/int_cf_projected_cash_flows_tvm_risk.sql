WITH projected_cash_flows_common AS (
    SELECT
          *
    FROM {{ ref('int_cf_projected_cash_flows_common')}} cf
), grouped AS (
    SELECT
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , SUM(projected_disbursal_amount_in_cents) AS projected_disbursal_amount_in_cents
        , days_from_now
        , case when days_from_now < 0 then 0 else SUM(projected_payment_amount_in_cents) end AS projected_payment_amount_in_cents
    FROM projected_cash_flows_common
    GROUP BY
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , days_from_now
    ORDER BY days_from_now
), arrayed AS (
    SELECT
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , ARRAY_AGG(projected_disbursal_amount_in_cents) AS projected_disbursal_amount_in_cents
        , ARRAY_AGG(days_from_now) AS days_from_now
        , ARRAY_AGG(projected_payment_amount_in_cents) AS cash_flows
    FROM grouped
    GROUP BY
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
)
, with_risk AS (
    SELECT
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , projected_disbursal_amount_in_cents
        , days_from_now
        , cash_flows
        , {{target.schema}}.udf_loan_pv(bench_mark_daily_interest_rate, days_from_now, projected_disbursal_amount_in_cents) AS disbursal_pv
        , {{target.schema}}.udf_loan_pv(bench_mark_daily_interest_rate, days_from_now, cash_flows) AS pv
        , SAFE_MULTIPLY({{target.schema}}.udf_loan_ytm(bench_mark_daily_interest_rate, days_from_now, cash_flows), 365.0) AS ytm
        , {{target.schema}}.udf_loan_mac_duration(bench_mark_daily_interest_rate, days_from_now, cash_flows) AS mac_duration
        , SAFE_DIVIDE({{target.schema}}.udf_loan_mod_duration(bench_mark_daily_interest_rate, days_from_now, cash_flows), 365.0) AS mod_duration
        , SAFE_DIVIDE({{target.schema}}.udf_loan_convexity(bench_mark_daily_interest_rate, days_from_now, cash_flows), 365.0 * 365.0) AS convexity
        , {{target.schema}}.udf_loan_pv_delta_on_interest_rate_delta_with_convex(bench_mark_daily_interest_rate, days_from_now, cash_flows, 0.0001 / days_per_year) AS dv01
        , {{target.schema}}.udf_loan_pv(bench_mark_daily_interest_rate + (0.0001 / days_per_year), days_from_now, cash_flows) AS pv_at_dv01
    FROM arrayed
), final AS (
    SELECT
          customer_id
        , event_id
        , credit_facility_start_date
        , credit_facility_end_date
        , now_ts
        , days_per_year
        , bench_mark_daily_interest_rate
        , projected_disbursal_amount_in_cents
        , days_from_now
        , cash_flows
        , SAFE_DIVIDE(disbursal_pv, 100.0) AS disbursal_pv
        , SAFE_DIVIDE(pv, 100.0) AS pv
        , SAFE_DIVIDE(SAFE_ADD({{target.schema}}.udf_loan_pv(bench_mark_daily_interest_rate, days_from_now, cash_flows), disbursal_pv), 100.0) AS npv
        , ytm
        , SAFE_MULTIPLY({{target.schema}}.udf_loan_ytm_from_price(SAFE_NEGATE(disbursal_pv), days_from_now, cash_flows), 365.0) AS ytm_from_price
        , mac_duration
        , CASE WHEN IS_NAN(mac_duration)
            THEN TIMESTAMP('1900-01-01')
            ELSE TIMESTAMP(TIMESTAMP_ADD(DATE(now_ts), INTERVAL CAST(mac_duration AS INT64) DAY))
        END AS mac_duration_date
        , SAFE_DIVIDE(dv01, 100.0) AS dv01
        , SAFE_DIVIDE(pv_at_dv01, 100.0) AS pv_at_dv01
    FROM with_risk
)

SELECT * FROM final
