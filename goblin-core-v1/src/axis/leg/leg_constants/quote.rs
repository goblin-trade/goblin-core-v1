use crate::{
    axis::leg::{LegConstants, Quote},
    quantities::{Position, QuoteAtomsPerQuoteUnit, ATOMS_PER_UNIT},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: Position = Position::MAX;
}
