use crate::{
    quantities::{QuantityOps, Ticks},
    types::{LegValidator, Quote},
};

impl LegValidator for Quote {
    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }
}
