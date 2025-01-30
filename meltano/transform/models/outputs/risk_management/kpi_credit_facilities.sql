with this as (
    select
        name,
        value,
        order_by,
        1 as rpt_order
    from {{ ref('int_customers_in_numbers') }}
    union all
    select
        name,
        value,
        order_by,
        2 as rpt_order
    from {{ ref('int_credit_facilities_in_numbers') }}
    union all
    select
        name,
        value,
        order_by,
        3 as rpt_order
    from {{ ref('int_credit_facilities_in_values') }}
    union all
    select
        name,
        value,
        order_by,
        4 as rpt_order
    from {{ ref('int_credit_facilities_in_time_value_of_money') }}
    order by rpt_order, order_by
)

select
    name,
    value
from this
