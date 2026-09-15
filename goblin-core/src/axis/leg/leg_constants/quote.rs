use crate::{
    axis::leg::{LegConstants, Quote},
    quantities::{ATOMS_PER_UNIT, FullPosition, PositionV2, QuoteAtomsPerQuoteUnit},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: PositionV2 = PositionV2::MAX;
}
