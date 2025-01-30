
WITH total_customers AS (
  SELECT
    COUNT(DISTINCT customer_id) AS value
  FROM {{ref("int_credit_facilities")}}
), total_active_customers AS (
  SELECT
    COUNT(DISTINCT customer_id) AS value
  FROM {{ref("int_credit_facilities")}}
  WHERE completed_recorded_at IS NULL
), approved_cf AS (
  SELECT
    COUNT(DISTINCT customer_id) AS value
  FROM {{ref("int_credit_facilities")}}
  WHERE approval_process_concluded_approved
), activated_cf AS (
  SELECT
    COUNT(DISTINCT customer_id) AS value
  FROM {{ref("int_credit_facilities")}}
  WHERE activated_recorded_at_date_key != 19000101
), disbursed_cf AS (
  SELECT
    COUNT(DISTINCT customer_id) AS value
  FROM {{ref("int_cf_denormalized")}}
  WHERE disbursal_concluded_event_recorded_at_date_key != 19000101
)


SELECT 1 AS order_by, CAST(value AS STRING) AS value, 'Total Number of Customers' AS name FROM total_customers
  UNION ALL
SELECT 1 AS order_by, CAST(value AS STRING) AS value, 'Total Number of Active Customers' AS name FROM total_active_customers
  UNION ALL
SELECT 2 AS order_by, CAST(value AS STRING) AS value, 'Total Number of Customers with Approved Credit Facilities' AS name FROM approved_cf
  UNION ALL
SELECT 3 AS order_by, CAST(value AS STRING) AS value, 'Total Number of Customers with Disbursed Approved Credit Facilities' AS name FROM disbursed_cf

ORDER BY order_by
