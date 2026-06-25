use crate::{
    axis::leg::{leg_constants::LegConstants, Quote},
    quantities::{Position, QuoteAtomsPerQuoteUnit, ATOMS_PER_UNIT},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit =
        QuoteAtomsPerQuoteUnit::new(ATOMS_PER_UNIT.inner as u64);
    const DEFAULT_PRICE_LIMIT: Position = Position::MAX;
}
