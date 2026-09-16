use crate::{
    axis::leg::{LegConstants, Quote},
    quantities::{ATOMS_PER_UNIT, FullPosU32, FullPositionU32, QuoteAtomsPerQuoteUnit},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QuoteAtomsPerQuoteUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: FullPosU32 = FullPosU32::MAX;
}
