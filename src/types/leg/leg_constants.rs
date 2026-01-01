use crate::{
    quantities::{BaseAtomsPerBaseUnit, QuantityOps, QuoteAtomsPerQuoteUnit, Ticks},
    types::{Base, LegQuantities, Quote},
};

pub trait LegConstants: LegQuantities {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
    const DEFAULT_PRICE_LIMIT: Ticks;
}

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;
}

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;
}
