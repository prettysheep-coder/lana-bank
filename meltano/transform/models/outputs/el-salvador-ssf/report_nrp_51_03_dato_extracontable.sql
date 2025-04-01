{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_extracontable`, 10) AS `id_codigo_extracontable`
  , LEFT(`desc_extra_contable`, 80) AS `desc_extra_contable`
  , CAST(ROUND(`Valor`, 2) AS STRING) AS `Valor`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_03_dato_extracontable') }}
