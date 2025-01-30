with total_customers as (
    select count(distinct customer_id) as value
    from {{ ref("int_credit_facilities") }}
),

total_active_customers as (
    select count(distinct customer_id) as value
    from {{ ref("int_credit_facilities") }}
    where completed_recorded_at is null
),

approved_cf as (
    select count(distinct customer_id) as value
    from {{ ref("int_credit_facilities") }}
    where approval_process_concluded_approved
),

activated_cf as (
    select count(distinct customer_id) as value
    from {{ ref("int_credit_facilities") }}
    where activated_recorded_at_date_key != 19000101
),

disbursed_cf as (
    select count(distinct customer_id) as value
    from {{ ref("int_cf_denormalized") }}
    where disbursal_concluded_event_recorded_at_date_key != 19000101
)


select
    1 as order_by,
    cast(value as string) as value,
    'Total Number of Customers' as name
from total_customers
union all
select
    1 as order_by,
    cast(value as string) as value,
    'Total Number of Active Customers' as name
from total_active_customers
union all
select
    2 as order_by,
    cast(value as string) as value,
    'Total Number of Customers with Approved Credit Facilities' as name
from approved_cf
union all
select
    3 as order_by,
    cast(value as string) as value,
    'Total Number of Customers with Disbursed Approved Credit Facilities'
        as name
from disbursed_cf

order by order_by
