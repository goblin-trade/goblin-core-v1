use crate::{
    quantities::{QuantityOps, QuoteAtomsPerQuoteUnit, Ticks},
    types::{LegConstants, Quote},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;
}
