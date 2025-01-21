
SELECT account_set_id,
	member_account_id AS member_id,
	"Account" AS member_type

FROM {{ ref('stg_account_set_member_accounts') }}

UNION ALL

SELECT account_set_id,
	member_account_id AS member_id,
	"AccountSet" AS member_type

FROM {{ ref('stg_account_set_member_accounts') }}
