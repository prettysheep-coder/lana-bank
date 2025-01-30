WITH payment_recorded AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , event_type
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_in_ledger_at"), "UTC")) as INT64) AS recorded_in_ledger_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_in_ledger_at"), "UTC") AS recorded_in_ledger_at
        , CAST(JSON_VALUE(event, "$.disbursement_amount") AS NUMERIC) AS disbursement_amount
        , CAST(JSON_VALUE(event, "$.interest_amount") AS NUMERIC) AS interest_amount
    FROM {{ ref('stg_credit_facility_events') }} AS cfe
    WHERE cfe.event_type = "payment_recorded"
    AND JSON_VALUE(event, "$.tx_id") IS NOT NULL

)


SELECT
      *
FROM payment_recorded
