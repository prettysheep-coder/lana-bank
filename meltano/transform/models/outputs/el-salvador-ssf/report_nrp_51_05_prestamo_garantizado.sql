{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_banco`, 10) AS `id_codigo_banco`
  , LEFT(`nom_banco`, 80) AS `nom_banco`
  , LEFT(`Pais`, 20) AS `Pais`
  , LEFT(`categoria`, 2) AS `categoria`
  , CAST(ROUND(`valor`, 2) AS STRING) AS `valor`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_05_prestamo_garantizado') }}
