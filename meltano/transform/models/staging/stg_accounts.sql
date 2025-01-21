select
    id,
    code,
    name,
    normal_balance_type,
    latest_values,
    created_at

from {{ source("lana", "public_cala_accounts_view") }}
