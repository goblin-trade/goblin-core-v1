use crate::{
    axis::leg::{LegConstants, Quote},
    quantities::{ATOMS_PER_UNIT, FullPosU32, FullPositionU32, QuoteAtomsPerQuoteUnit, U32Variant},
};

impl LegConstants for Quote {
    const ATOMS_PER_UNIT: U32Variant<Self::AtomsPerUnit> =
        QuoteAtomsPerQuoteUnit::new(ATOMS_PER_UNIT.inner as u32);
    const DEFAULT_PRICE_LIMIT: FullPosU32 = FullPosU32::MAX;
}
