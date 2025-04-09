{{ config(materialized='table') }}

select
    LEFT(`Código del Producto`, 4) AS `Código del Producto`
  , LEFT(`Número de cuenta`, 20) AS `Número de cuenta`
  , LEFT(`Agencia`, 7) AS `Agencia`
  , LEFT(`Tipo de Periodicidad`, 1) AS `Tipo de Periodicidad`
  , CAST(ROUND(`Tasa vigente`, 2) AS STRING) AS `Tasa vigente`
  , CAST(ROUND(`Tasa inicial`, 2) AS STRING) AS `Tasa inicial`
  , FORMAT_DATE('%Y%m%d', CAST(`Fecha inicial de tasa` AS DATE)) AS `Fecha inicial de tasa`
  , FORMAT_DATE('%Y%m%d', CAST(`Fecha fin de tasa` AS DATE)) AS `Fecha fin de tasa`
  , LEFT(`Tipo de tasa`, 2) AS `Tipo de tasa`
  , LEFT(`Forma de pago de interés`, 2) AS `Forma de pago de interés`
  , CAST(ROUND(`Tasa de referencia`, 2) AS STRING) AS `Tasa de referencia`
  , CAST(ROUND(`Porcentaje a pagar por intereses`, 2) AS STRING) AS `Porcentaje a pagar por intereses`
  , FORMAT_DATE('%Y%m%d', CAST(`Día de corte` AS DATE)) AS `Día de corte`
  , CAST(ROUND(`Porcentaje de comisión`, 2) AS STRING) AS `Porcentaje de comisión`
  , LEFT(`Tipo de titularidad`, 1) AS `Tipo de titularidad`
  , CAST(`Número de titulares` AS STRING) AS `Número de titulares`
  , LEFT(`Plazo de la Cuenta`, 8) AS `Plazo de la Cuenta`
  , LEFT(`Condiciones especiales`, 1) AS `Condiciones especiales`
  , LEFT(`Explicación de condiciones especiales`, 100) AS `Explicación de condiciones especiales`
  , FORMAT_DATE('%Y%m%d', CAST(`Fecha de apertura` AS DATE)) AS `Fecha de apertura`
  , FORMAT_DATE('%Y%m%d', CAST(`Fecha de vencimiento` AS DATE)) AS `Fecha de vencimiento`
  , CAST(ROUND(`Monto mínimo`, 2) AS STRING) AS `Monto mínimo`
  , LEFT(`Código de la cuenta contable`, 20) AS `Código de la cuenta contable`
  , CAST(ROUND(`Fondos en compensación`, 2) AS STRING) AS `Fondos en compensación`
  , CAST(ROUND(`Fondos restringidos`, 2) AS STRING) AS `Fondos restringidos`
  , CAST(ROUND(`Transacciones pendientes`, 2) AS STRING) AS `Transacciones pendientes`
  , LEFT(`Negociabilidad del depósito`, 1) AS `Negociabilidad del depósito`
  , LEFT(`Moneda`, 3) AS `Moneda`
  , CAST(ROUND(`Saldo del depósito en la moneda original`, 2) AS STRING) AS `Saldo del depósito en la moneda original`
  , FORMAT_DATE('%Y%m%d', CAST(`Fecha de la última transacción` AS DATE)) AS `Fecha de la última transacción`
  , CAST(ROUND(`Saldo de capital`, 2) AS STRING) AS `Saldo de capital`
  , CAST(ROUND(`Saldo de intereses`, 2) AS STRING) AS `Saldo de intereses`
  , CAST(ROUND(`Saldo total`, 2) AS STRING) AS `Saldo total`
  , LEFT(`Estado`, 1) AS `Estado`
from
  {{ ref('int_nrsf_03_02_depósitos') }}
