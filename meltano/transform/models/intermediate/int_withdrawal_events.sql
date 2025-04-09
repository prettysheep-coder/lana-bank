select distinct
    json_value(parsed_event.id) as withdrawal_id,
    event_type,
    cast(json_value(parsed_event.amount) as numeric) as amount,
    cast(json_value(parsed_event.approved) as boolean) as approved,
    json_value(parsed_event.reference) as reference,
    json_value(parsed_event.deposit_account_id) as deposit_account_id,
    json_value(parsed_event.approval_process_id) as approval_process_id,
    recorded_at

from {{ ref('stg_withdrawal_events') }}

where event_type = "initialized"
