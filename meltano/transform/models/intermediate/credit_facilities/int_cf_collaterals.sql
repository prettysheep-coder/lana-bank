WITH collateral_updated AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , event_type
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_in_ledger_at"), "UTC")) as INT64) AS recorded_in_ledger_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_in_ledger_at"), "UTC") AS recorded_in_ledger_at
        , JSON_VALUE(event, "$.action") AS action
        , CAST(JSON_VALUE(event, "$.audit_info.audit_entry_id") AS INTEGER) AS audit_entry_id
        , CAST(JSON_VALUE(event, "$.abs_diff") AS NUMERIC) AS abs_diff
        , CAST(JSON_VALUE(event, "$.total_collateral") AS NUMERIC) AS total_collateral
    FROM {{ ref('stg_credit_facility_events') }} AS cfe
    WHERE cfe.event_type = "collateral_updated"
    AND JSON_VALUE(event, "$.tx_id") IS NOT NULL

), collateralization_changed AS (

    SELECT
          id AS event_id
        , CAST(FORMAT_DATE('%Y%m%d', recorded_at) as INT64) AS recorded_at_date_key
        , recorded_at
        , CAST(FORMAT_DATE('%Y%m%d', PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_at"), "UTC")) as INT64) AS event_recorded_at_date_key
        , PARSE_TIMESTAMP('%Y-%m-%dT%H:%M:%E*SZ', JSON_VALUE(event, "$.recorded_at"), "UTC") AS event_recorded_at
        , JSON_VALUE(event, "$.state") AS state
        , CAST(JSON_VALUE(event, "$.audit_info.audit_entry_id") AS INTEGER) AS audit_entry_id
        , CAST(JSON_VALUE(event, "$.collateral") AS NUMERIC) AS collateral
        , CAST(JSON_VALUE(event, "$.price") AS NUMERIC) AS price
        , CAST(JSON_VALUE(event, "$.outstanding.disbursed") AS NUMERIC) AS outstanding_disbursed
        , CAST(JSON_VALUE(event, "$.outstanding.interest") AS NUMERIC) AS outstanding_interest
    FROM {{ ref('stg_credit_facility_events') }} cfe
    WHERE cfe.event_type = "collateralization_changed"

)


SELECT
      cu.* EXCEPT (abs_diff, total_collateral)

    , COALESCE(cc.event_recorded_at_date_key, 19000101) AS collateralization_changed_event_recorded_at_date_key
    , cc.event_recorded_at AS collateralization_changed_event_recorded_at
    , state AS collateralization_changed_state

    , CASE WHEN LOWER(action) = 'add' THEN cu.abs_diff ELSE SAFE_NEGATE(cu.abs_diff) END as diff
    , cu.total_collateral

    , COALESCE(cc.collateral, 0) AS collateral
    , cc.price
    , COALESCE(cc.outstanding_disbursed, 0) AS outstanding_disbursed
    , COALESCE(cc.outstanding_interest, 0) AS outstanding_interest
FROM collateral_updated AS cu
LEFT JOIN collateralization_changed cc ON cc.event_id = cu.event_id AND cc.audit_entry_id = cu.audit_entry_id
