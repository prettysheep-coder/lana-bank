{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_deuda`, 10) AS `id_codigo_deuda`
  , LEFT(`desc_deuda`, 80) AS `desc_deuda`
  , CAST(ROUND(`valor_deuda`, 2) AS STRING) AS `valor_deuda`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_07_deuda_subordinada') }}
