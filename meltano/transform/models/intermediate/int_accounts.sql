WITH all_accounts AS (

	SELECT
		id AS account_id,
		name AS account_name,
		normal_balance_type,
		code AS account_code,
		LAX_BOOL(PARSE_JSON(JSON_VALUE(latest_values, "$.config.is_account_set"))) AS is_account_set,

	FROM {{ ref('stg_accounts') }}

), credit_facilities AS (

	SELECT DISTINCT credit_facility_key,
		collateral_account_id,
		disbursed_receivable_account_id,
		facility_account_id,
		fee_income_account_id,
		interest_account_id,
		interest_receivable_account_id,

	FROM {{ ref('int_approved_credit_facilities') }}

), credit_facility_accounts AS (

	SELECT DISTINCT credit_facility_key,
		collateral_account_id AS account_id,
		"collateral_account" AS account_type,
	FROM credit_facilities

	UNION DISTINCT

	SELECT DISTINCT credit_facility_key,
		disbursed_receivable_account_id AS account_id,
		"disbursed_receivable_account" AS account_type,
	FROM credit_facilities

	UNION DISTINCT

	SELECT DISTINCT credit_facility_key,
		facility_account_id AS account_id,
		"facility_account" AS account_type,
	FROM credit_facilities

	UNION DISTINCT

	SELECT DISTINCT credit_facility_key,
		fee_income_account_id AS account_id,
		"fee_income_account" AS account_type,
	FROM credit_facilities

	UNION DISTINCT

	SELECT DISTINCT credit_facility_key,
		interest_account_id AS account_id,
		"interest_account" AS account_type,
	FROM credit_facilities

	UNION DISTINCT

	SELECT DISTINCT credit_facility_key,
		interest_receivable_account_id AS account_id,
		"interest_receivable_account" AS account_type,
	FROM credit_facilities

)

SELECT account_id, account_name, normal_balance_type, account_code, is_account_set,
	credit_facility_key, account_type,
	ROW_NUMBER() OVER() AS account_key,

FROM all_accounts
LEFT JOIN credit_facility_accounts USING (account_id)
