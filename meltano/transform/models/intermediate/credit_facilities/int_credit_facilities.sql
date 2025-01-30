WITH initialized AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , event_type
        , JSON_VALUE(event, "$.customer_id") AS customer_id
        , JSON_VALUE(event, "$.terms.accrual_interval.type") AS terms_accrual_interval_type
        , CAST(JSON_VALUE(event, "$.terms.annual_rate") AS NUMERIC) AS terms_annual_rate
        , JSON_VALUE(event, "$.terms.duration.type") AS terms_duration_type
        , CAST(JSON_VALUE(event, "$.terms.duration.value") AS INTEGER) AS terms_duration_value
        , JSON_VALUE(event, "$.terms.incurrence_interval.type") AS terms_incurrence_interval_type
        , CAST(JSON_VALUE(event, "$.terms.initial_cvl") AS NUMERIC) AS terms_initial_cvl
        , CAST(JSON_VALUE(event, "$.terms.liquidation_cvl") AS NUMERIC) AS terms_liquidation_cvl
        , CAST(JSON_VALUE(event, "$.terms.margin_call_cvl") AS NUMERIC) AS terms_margin_call_cvl
        , CAST(JSON_VALUE(event, "$.facility") AS NUMERIC) AS facility
    FROM {{ ref('stg_credit_facility_events') }} AS cfe
    WHERE cfe.event_type = "initialized"

), approval_process_started AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "approval_process_started"

), approval_process_concluded AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , CAST(JSON_VALUE(event, "$.approved") AS BOOLEAN) AS approved
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "approval_process_concluded"

), activated AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.activated_at")), "UTC") as INT64) AS activated_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.activated_at"), "UTC") AS activated_at
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "activated"

), completed AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.completed_at")), "UTC") as INT64) AS completed_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.completed_at"), "UTC") AS completed_at
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "completed"

)


SELECT
      i.* EXCEPT (facility)

    , COALESCE(aps.recorded_at_date_key, 19000101) AS approval_process_started_recorded_at_date_key
    , aps.recorded_at AS approval_process_started_recorded_at

    , COALESCE(apc.recorded_at_date_key, 19000101) AS approval_process_concluded_recorded_at_date_key
    , apc.recorded_at AS approval_process_concluded_recorded_at
    , COALESCE(apc.approved, FALSE) AS approval_process_concluded_approved

    , COALESCE(a.recorded_at_date_key, 19000101) AS activated_recorded_at_date_key
    , a.recorded_at AS activated_recorded_at
    , COALESCE(a.activated_at_date_key, 19000101) AS activated_at_date_key
    , a.activated_at

    , COALESCE(c.recorded_at_date_key, 19000101) AS completed_recorded_at_date_key
    , c.recorded_at AS completed_recorded_at
    , COALESCE(c.completed_at_date_key, 19000101) AS completed_at_date_key
    , c.completed_at

    , i.facility
FROM initialized AS i
LEFT JOIN approval_process_started aps ON aps.event_id = i.event_id
LEFT JOIN approval_process_concluded apc ON apc.event_id = i.event_id
LEFT JOIN activated a ON a.event_id = i.event_id
LEFT JOIN completed c ON c.event_id = i.event_id
