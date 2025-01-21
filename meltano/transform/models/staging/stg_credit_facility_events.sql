
SELECT
	id,
	sequence,
	event_type,
	event,
	recorded_at,

FROM {{ source("lana", "public_credit_facility_events_view") }}
