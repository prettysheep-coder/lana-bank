WITH final AS (
  SELECT * FROM {{ref("int_cf_agg_projected_cash_flows_tvm_risk")}}
)


SELECT 0 AS order_by, CAST(disbursal_pv AS STRING) AS value, 'Present Value of disbursal cashflows' AS name FROM final
    UNION ALL
SELECT 1 AS order_by, CAST(pv AS STRING), 'Present Value of future cashflows' AS name FROM final
    UNION ALL
SELECT 2 AS order_by, CAST(npv AS STRING), 'Net Present Value of disbursal & future cashflows' FROM final
    UNION ALL
SELECT 3 AS order_by, CAST(ytm AS STRING), 'YTM' FROM final
    UNION ALL
SELECT 4 AS order_by, CAST(ytm_from_price AS STRING), 'YTM @ disbursal pv' FROM final
    UNION ALL
SELECT 5 AS order_by, CAST(mac_duration AS STRING), 'MacDuration' FROM final
    UNION ALL
SELECT 6 AS order_by, CAST(mac_duration_date AS STRING), 'MacDurationDate' FROM final
    UNION ALL
SELECT 7 AS order_by, CAST(dv01 AS STRING), 'DV01' FROM final
    UNION ALL
SELECT 8 AS order_by, CAST(pv_at_dv01 AS STRING), 'PV @ DV01' FROM final

ORDER BY order_by
