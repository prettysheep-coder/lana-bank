with approved as (
    select count(distinct event_id) as value
    from {{ ref("int_credit_facilities") }}
    where approval_process_concluded_approved
),

total as (
    select count(distinct event_id) as value
    from {{ ref("int_credit_facilities") }}
)


select
    1 as order_by,
    cast(value as string) as value,
    'Number of Approved Credit Facilities' as name
from approved
union all
select
    2 as order_by,
    cast(value as string) as value,
    'Number of Total Credit Facilities'
from total
union all
select
    3 as order_by,
    cast(a.value / t.value as string) as value,
    'Approved Rate'
from approved as a, total as t

order by order_by
