WITH disbursal_initiated AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , event_type
        , CAST(JSON_VALUE(event, "$.amount") AS NUMERIC) AS amount
        , CAST(JSON_VALUE(event, "$.idx") AS INTEGER) AS idx
    FROM {{ ref('stg_credit_facility_events') }} AS cfe
    WHERE cfe.event_type = "disbursal_initiated"

), disbursal_concluded AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_at"), "UTC")) as INT64) AS event_recorded_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_at"), "UTC") AS event_recorded_at
        , CAST(JSON_VALUE(event, "$.idx") AS INTEGER) AS idx
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "disbursal_concluded"
    AND JSON_VALUE(event, "$.tx_id") IS NOT NULL

)


SELECT
      di.* EXCEPT (amount)

    , COALESCE(dc.event_recorded_at_date_key, 19000101) AS disbursal_concluded_event_recorded_at_date_key
    , dc.event_recorded_at AS disbursal_concluded_event_recorded_at

    , di.amount
FROM disbursal_initiated AS di
LEFT JOIN disbursal_concluded dc ON dc.event_id = di.event_id AND di.idx = dc.idx
