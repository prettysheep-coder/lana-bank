{{ config(materialized='table') }}

SELECT
    LEFT(`id_codigo_banco`, 10) AS `id_codigo_banco`
  , LEFT(`nom_banco`, 80) AS `nom_banco`
  , LEFT(`Pais`, 20) AS `Pais`
  , LEFT(`Categoria`, 2) AS `Categoria`
  , CAST(ROUND(`Valor`, 2) AS STRING) AS `Valor`
  , CURRENT_TIMESTAMP() AS created_at
FROM
  {{ ref('int_nrp_51_02_deposito_extranjero') }}
