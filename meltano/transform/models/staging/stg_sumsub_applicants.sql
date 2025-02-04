select *

from {{ source("lana", "applicants_view") }}
