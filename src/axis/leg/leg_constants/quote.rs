use crate::{
    axis::leg::{leg_constants::LegConstants, Quote},
    quantities::{QuantityOps, QuoteAtomsPerQuoteUnit, Ticks},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;
}
