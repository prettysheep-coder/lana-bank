SELECT
		RIGHT(id_codigo_cuenta, 10) AS id_codigo_cuenta,
		LEFT(nom_cuenta, 80) AS nom_cuenta,
		FORMAT('%.2f', ROUND(valor, 2)) AS valor,

FROM {{ ref('int_npb4_16_01_saldo_cuenta_xml_raw') }}
