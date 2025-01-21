
SELECT
	journal_id,
	account_id,
	currency,
	recorded_at,
	values,

FROM {{ source("lana", "public_cala_balance_history_view") }}
