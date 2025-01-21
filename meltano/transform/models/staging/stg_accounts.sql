
SELECT
	id,
	code,
	name,
	normal_balance_type,
	latest_values,
	created_at,

FROM {{ source("lana", "public_cala_accounts_view") }}
