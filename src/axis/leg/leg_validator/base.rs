use crate::{
    axis::leg::{leg_validator::LegValidator, Base},
    quantities::Ticks,
};

impl LegValidator for Base {
    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }
}
