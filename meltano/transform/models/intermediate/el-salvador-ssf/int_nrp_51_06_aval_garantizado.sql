WITH

accounts AS (
  SELECT *
  FROM
    {{ ref('stg_accounts') }}
)
,

account_sets AS (
  SELECT *
  FROM
    {{ ref('stg_account_sets') }}
)
,

account_set_member_accounts AS (
  SELECT *
  FROM
    {{ ref('stg_account_set_member_accounts') }}
)
,

account_set_member_account_sets AS (
  SELECT *
  FROM
    {{ ref('stg_account_set_member_account_sets') }}
)
,

account_balances AS (
  SELECT *
  FROM
    {{ ref('stg_account_balances') }}
)
,

bitfinex_order_book AS (
  SELECT *
  FROM
    {{ ref('stg_bitfinex_order_book') }}
)
,

bitfinex_ticker_price AS (
  SELECT *
  FROM
    {{ ref('stg_bitfinex_ticker_price') }}
)
,

bitfinex_trades AS (
  SELECT *
  FROM
    {{ ref('stg_bitfinex_trades') }}
)
,

credit_facility_events AS (
  SELECT *
  FROM
    {{ ref('stg_credit_facility_events') }}
)
,

customer_events AS (
  SELECT *
  FROM
    {{ ref('stg_customer_events') }}
)
,

sumsub_applicants AS (
  SELECT *
  FROM
    {{ ref('stg_sumsub_applicants') }}
)


SELECT
    'TODO' AS `id_codigo_banco`
  , 'TODO' AS `nom_banco`
  , 'TODO' AS `Pais`
  , 'TODO' AS `categoria`
  , 7060.0 AS `valor_aval_fianza`
-- FROM
--     accounts,
--     account_sets,
--     account_set_member_accounts,
--     account_set_member_account_sets,
--     account_balances,
--     bitfinex_order_book,
--     bitfinex_ticker_price,
--     bitfinex_trades,
--     credit_facility_events,
--     customer_events,
--     sumsub_applicants,
