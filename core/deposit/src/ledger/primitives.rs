use core_money::UsdCents;

pub struct LayeredUsdBalance {
    pub settled: UsdCents,
    pub pending: UsdCents,
}

impl LayeredUsdBalance {
    pub const ZERO: Self = LayeredUsdBalance {
        settled: UsdCents::ZERO,
        pending: UsdCents::ZERO,
    };
}
