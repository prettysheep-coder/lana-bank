{{ config(materialized='table') }}

select
    right(`id_codigo_cuenta`, 10) AS `id_codigo_cuenta`
  , LEFT(regexp_replace(`nom_cuenta`, r'[&<>"]', '_'), 80) AS `nom_cuenta`
  , CAST(format('%.2f', ROUND(`valor`, 2)) AS STRING) AS `valor`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_01_saldo_cuenta') }}
