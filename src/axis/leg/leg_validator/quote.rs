use crate::{
    axis::leg::{leg_validator::LegValidator, Quote},
    quantities::{QuantityOps, Ticks},
};

impl LegValidator for Quote {
    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }
}
