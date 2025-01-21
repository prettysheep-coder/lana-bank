select
    id,
    sequence,
    event_type,
    event,
    recorded_at

from {{ source("lana", "public_credit_facility_events_view") }}
