
SELECT
	id AS account_set_id,
	ROW_NUMBER() OVER() AS set_key,
	name AS set_name,

FROM {{ ref('stg_account_sets') }}
