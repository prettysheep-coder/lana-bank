
WITH approved AS (
  SELECT
    COUNT(DISTINCT event_id)  AS value
  FROM {{ref("int_credit_facilities")}}
  WHERE approval_process_concluded_approved
), total AS (
  SELECT
    COUNT(DISTINCT event_id)  AS value
  FROM {{ref("int_credit_facilities")}}
)


SELECT 1 AS order_by, CAST(value AS STRING) AS value, 'Number of Approved Credit Facilities' AS name FROM approved
  UNION ALL
SELECT 2 AS order_by, CAST(value AS STRING) AS value, 'Number of Total Credit Facilities' FROM total
  UNION ALL
SELECT 3 AS order_by, CAST(a.value / t.value AS STRING) AS value, 'Approved Rate' FROM approved a, total t

ORDER BY order_by
