with final as (
    select * from {{ ref("int_cf_agg_projected_cash_flows_tvm_risk") }}
)


select
    0 as order_by,
    cast(disbursal_pv as string) as value,
    'Present Value of disbursal cashflows' as name
from final
union all
select
    1 as order_by,
    cast(pv as string),
    'Present Value of future cashflows' as name
from final
union all
select
    2 as order_by,
    cast(npv as string),
    'Net Present Value of disbursal & future cashflows'
from final
union all
select
    3 as order_by,
    cast(ytm as string),
    'YTM'
from final
union all
select
    4 as order_by,
    cast(ytm_from_price as string),
    'YTM @ disbursal pv'
from final
union all
select
    5 as order_by,
    cast(mac_duration as string),
    'MacDuration'
from final
union all
select
    6 as order_by,
    cast(mac_duration_date as string),
    'MacDurationDate'
from final
union all
select
    7 as order_by,
    cast(dv01 as string),
    'DV01'
from final
union all
select
    8 as order_by,
    cast(pv_at_dv01 as string),
    'PV @ DV01'
from final

order by order_by
