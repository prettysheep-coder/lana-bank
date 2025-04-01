{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_titulo_extranjero`, 10) AS `id_codigo_titulo_extranjero`
  , LEFT(`desc_tv_extranj`, 254) AS `desc_tv_extranj`
  , CAST(ROUND(`valor_tv_extranj`, 2) AS STRING) AS `valor_tv_extranj`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_04_titulo_valor_extranjero') }}
