use crate::{
    quantities::{QuantityOps, Ticks},
    types::{Base, LegMarker, Quote},
};

pub trait LegValidator: LegMarker {
    /// Ensure that market has an integer number of atoms per lot
    ///
    /// As ATOMS_PER_UNIT is hardcoded to 10^6 for base and quote, this is effectively
    ///
    ///  **10^6 % Lot size == 0**
    ///
    /// lots_per_unit is also called lot_size
    fn lots_per_unit_valid(lots_per_unit: Self::LotsPerUnit) -> bool {
        Self::ATOMS_PER_UNIT % lots_per_unit == Self::AtomsPerUnit::ZERO
    }

    fn price_limit_valid(_price_limit: Ticks) -> bool;
}

impl LegValidator for Base {
    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }
}

impl LegValidator for Quote {
    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }
}
