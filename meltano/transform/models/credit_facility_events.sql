SELECT *

FROM {{ source("lana", "public_credit_facility_events_view") }}
