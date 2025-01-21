SELECT
	JSON_VALUE(values, "$.account_id") AS account_id,
	JSON_VALUE(values, "$.currency") AS currency,
	CAST(JSON_VALUE(ANY_VALUE(values HAVING MAX recorded_at), "$.settled.cr_balance") AS NUMERIC) AS settled_cr,
	CAST(JSON_VALUE(ANY_VALUE(values HAVING MAX recorded_at), "$.settled.dr_balance") AS NUMERIC) AS settled_dr,

FROM {{ ref('stg_account_balances') }}

GROUP BY account_id, currency
