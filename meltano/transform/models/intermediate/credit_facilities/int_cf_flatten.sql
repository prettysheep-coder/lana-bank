with credit_facilities as (

    select * from {{ ref('int_credit_facilities') }}

), int_cf_disbursals as (

    select
          event_id
        , max(recorded_at_date_key) as disbursal_recorded_at_date_key
        , max(recorded_at) as disbursal_recorded_at
        , max(disbursal_concluded_event_recorded_at_date_key) as disbursal_concluded_event_recorded_at_date_key
        , max(disbursal_concluded_event_recorded_at) as disbursal_concluded_event_recorded_at
        , sum(amount) as total_disbursed_amount
    from {{ ref('int_cf_disbursals') }}
    group by event_id

), int_cf_collaterals as (

    select
          event_id
        , max(recorded_at_date_key) as collateral_recorded_at_date_key
        , max(recorded_at) as collateral_recorded_at

        , max(recorded_in_ledger_at_date_key) as recorded_in_ledger_at_date_key
        , max(recorded_in_ledger_at) as recorded_in_ledger_at
        , max(collateralization_changed_event_recorded_at_date_key) as collateralization_changed_event_recorded_at_date_key
        , max(collateralization_changed_event_recorded_at) as collateralization_changed_event_recorded_at

        , ARRAY_AGG(collateralization_changed_state ORDER BY collateralization_changed_event_recorded_at DESC LIMIT 1)[SAFE_ORDINAL(1)] as collateralization_changed_state

        , sum(diff) as total_collateral_summed
        , ARRAY_AGG(total_collateral ORDER BY recorded_at DESC LIMIT 1)[SAFE_ORDINAL(1)] as total_collateral

        , sum(outstanding_disbursed) as outstanding_disbursed
        , sum(outstanding_interest) as outstanding_interest
    from {{ ref('int_cf_collaterals') }}
    group by event_id

), int_cf_payments as (

    select
          event_id
        , max(recorded_at_date_key) as payment_recorded_at_date_key
        , max(recorded_at) as payment_recorded_at
        , max(recorded_in_ledger_at_date_key) as payment_recorded_in_ledger_at_date_key
        , max(recorded_in_ledger_at) as payment_recorded_in_ledger_at
        , sum(disbursement_amount) as disbursement_amount
        , sum(interest_amount) as interest_amount
    from {{ ref('int_cf_payments') }}
    group by event_id

)

select
      cfe.*
    , d.* except(event_id)
    , c.* except(event_id)
    , p.* except(event_id)
from credit_facilities cfe
full join int_cf_disbursals d on d.event_id = cfe.event_id
full join int_cf_collaterals c on c.event_id = cfe.event_id
full join int_cf_payments p on p.event_id = cfe.event_id
