WITH terms_and_disbursal AS (
    SELECT
          *
        , facility AS credit_facility_limit_in_cents
        , SAFE_DIVIDE(terms_annual_rate, 100.0) AS credit_facility_annual_interest_rate
        , 'actual/360' AS credit_facility_day_count_convention	  -- TODO get from proper source
        , 5.53 / 100.0         AS bench_mark_interest_rate	      -- TODO get from proper source
        , TIMESTAMP(CURRENT_DATE()) AS now_ts
        , TIMESTAMP(DATE(activated_recorded_at)) AS credit_facility_start_date
        , CASE WHEN terms_duration_type = 'months' THEN
            TIMESTAMP(TIMESTAMP_ADD(DATE(activated_recorded_at), INTERVAL terms_duration_value MONTH))
            END AS credit_facility_end_date
        , amount AS disbursal_amount_in_cents
        , disbursal_concluded_event_recorded_at AS disbursal_start_date
    FROM {{ ref('int_cf_denormalized') }} cf
    WHERE disbursal_concluded_event_recorded_at_date_key != 19000101
    AND terms_accrual_interval_type = 'end_of_month'
), projections AS (
    SELECT
        *
    , SAFE_DIVIDE(
        credit_facility_annual_interest_rate,
        CASE
            WHEN ENDS_WITH(credit_facility_day_count_convention, '/360') THEN 360.0
            WHEN ENDS_WITH(credit_facility_day_count_convention, '/365') THEN 365.0
            ELSE TIMESTAMP_DIFF(TIMESTAMP(LAST_DAY(DATE(credit_facility_start_date), YEAR)), DATE_TRUNC(credit_facility_start_date, YEAR), DAY)
        END
        ) AS credit_facility_daily_interest_rate
    , SAFE_DIVIDE(
        bench_mark_interest_rate,
        CASE
            WHEN ENDS_WITH(credit_facility_day_count_convention, '/360') THEN 360.0
            WHEN ENDS_WITH(credit_facility_day_count_convention, '/365') THEN 365.0
            ELSE TIMESTAMP_DIFF(TIMESTAMP(LAST_DAY(DATE(credit_facility_start_date), YEAR)), DATE_TRUNC(credit_facility_start_date, YEAR), DAY)
        END
        ) AS bench_mark_daily_interest_rate
    , CASE
        WHEN ENDS_WITH(credit_facility_day_count_convention, '/360') THEN 360.0
        WHEN ENDS_WITH(credit_facility_day_count_convention, '/365') THEN 365.0
        ELSE TIMESTAMP_DIFF(TIMESTAMP(LAST_DAY(DATE(credit_facility_start_date), YEAR)), DATE_TRUNC(credit_facility_start_date, YEAR), DAY)
      END AS days_per_year
    , SAFE_DIVIDE(bench_mark_interest_rate, credit_facility_annual_interest_rate) AS breakeven_disbursal_percent
    , SAFE_MULTIPLY(
        credit_facility_limit_in_cents,
        SAFE_DIVIDE(bench_mark_interest_rate, credit_facility_annual_interest_rate)
        ) AS breakeven_disbursal_amount_in_cents
    , CASE
        WHEN terms_accrual_interval_type = 'end_of_day' THEN
            GENERATE_DATE_ARRAY(DATE(disbursal_start_date), LAST_DAY(DATE(credit_facility_end_date)), INTERVAL 1 DAY)
        WHEN terms_accrual_interval_type = 'end_of_month' THEN
            GENERATE_DATE_ARRAY(DATE(disbursal_start_date), LAST_DAY(DATE(credit_facility_end_date)), INTERVAL 1 MONTH)
      END AS interest_schedule_months
    FROM terms_and_disbursal
), projected_interest_payment_data AS (
    SELECT
          p.* except(interest_schedule_months)
        , CASE WHEN TIMESTAMP(DATE_TRUNC(projected_month, MONTH)) < disbursal_start_date THEN
                TIMESTAMP(DATE(disbursal_start_date))
            ELSE
                TIMESTAMP(DATE_TRUNC(projected_month, MONTH))
          END AS period_start_date
        , CASE WHEN LAST_DAY(projected_month) > DATE(credit_facility_end_date) THEN
                TIMESTAMP(DATE(credit_facility_end_date))
            ELSE
                TIMESTAMP(LAST_DAY(projected_month))
          END AS period_end_date
        , 'projected_interest_payment' AS payment_type
    FROM projections p,
            UNNEST(interest_schedule_months) AS projected_month
), projected_principal_payment_data AS (
    SELECT
          * except(interest_schedule_months)
        , TIMESTAMP(DATE(disbursal_start_date)) AS period_start_date
        , TIMESTAMP(DATE(credit_facility_end_date)) AS period_end_date
        , 'projected_principal_payment' AS payment_type
    FROM projections
), projected_disbursal_data AS (
    SELECT
          * except(interest_schedule_months)
        , TIMESTAMP(DATE(now_ts)) AS period_start_date
        , TIMESTAMP(TIMESTAMP_ADD(DATE(disbursal_start_date), INTERVAL -1 DAY)) AS period_end_date
        , 'projected_disbursal' AS payment_type
    FROM projections
), projected_payment_data AS (
    SELECT * FROM projected_interest_payment_data
        UNION ALL
    SELECT * FROM projected_principal_payment_data
        UNION ALL
    SELECT * FROM projected_disbursal_data
), projected_time_data AS (
    SELECT
          *
        , TIMESTAMP_DIFF(DATE(period_end_date), DATE(period_start_date), DAY) + 1 AS days_in_the_period
        , CAST(TIMESTAMP_DIFF(DATE(period_end_date), DATE(now_ts), DAY) + 1 AS FLOAT64) AS days_from_now
    FROM projected_payment_data
), projected_cash_flows_common AS (
    SELECT
          customer_id
        , event_id
        , idx AS disbursal_idx
        , credit_facility_start_date
        , credit_facility_end_date
        , bench_mark_interest_rate
        , bench_mark_daily_interest_rate
        , credit_facility_annual_interest_rate
        , credit_facility_daily_interest_rate
        , now_ts
        , days_per_year
        , days_in_the_period
        , days_from_now
        , CASE
            WHEN payment_type = 'projected_disbursal'
                THEN CAST(SAFE_NEGATE(disbursal_amount_in_cents) AS FLOAT64)
            ELSE 0
          END AS projected_disbursal_amount_in_cents
        , CASE
            WHEN payment_type = 'projected_interest_payment'
                THEN SAFE_MULTIPLY(disbursal_amount_in_cents, SAFE_MULTIPLY(credit_facility_daily_interest_rate, days_in_the_period))
            WHEN payment_type = 'projected_principal_payment'
                THEN disbursal_amount_in_cents
            ELSE 0
          END AS projected_payment_amount_in_cents
        FROM projected_time_data
)

SELECT
    *
FROM projected_cash_flows_common
