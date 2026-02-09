use crate::{
    quantities::Ticks,
    types::{Base, LegValidator},
};

impl LegValidator for Base {
    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }
}
