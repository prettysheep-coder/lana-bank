{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_cuentaproy`, 10) AS `id_codigo_cuentaproy`
  , LEFT(`nom_cuentaproy`, 80) AS `nom_cuentaproy`
  , CAST(ROUND(`enero`, 2) AS STRING) AS `enero`
  , CAST(ROUND(`febrero`, 2) AS STRING) AS `febrero`
  , CAST(ROUND(`marzo`, 2) AS STRING) AS `marzo`
  , CAST(ROUND(`abril`, 2) AS STRING) AS `abril`
  , CAST(ROUND(`mayo`, 2) AS STRING) AS `mayo`
  , CAST(ROUND(`junio`, 2) AS STRING) AS `junio`
  , CAST(ROUND(`julio`, 2) AS STRING) AS `julio`
  , CAST(ROUND(`agosto`, 2) AS STRING) AS `agosto`
  , CAST(ROUND(`septiembre`, 2) AS STRING) AS `septiembre`
  , CAST(ROUND(`octubre`, 2) AS STRING) AS `octubre`
  , CAST(ROUND(`noviembre`, 2) AS STRING) AS `noviembre`
  , CAST(ROUND(`diciembre`, 2) AS STRING) AS `diciembre`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_08_balance_proyectado') }}
