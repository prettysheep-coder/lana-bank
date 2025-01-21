
SELECT
	id,
	journal_id
	name,
	created_at,

FROM {{ source("lana", "public_cala_account_sets_view") }}
