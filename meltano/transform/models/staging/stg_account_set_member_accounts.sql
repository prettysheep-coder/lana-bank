
SELECT
	account_set_id,
	member_account_id,
	transitive,
	created_at,

FROM {{ source("lana", "public_cala_account_set_member_accounts_view") }}
