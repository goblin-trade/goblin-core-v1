use crate::{
    axis::leg::{leg_quantities::LegQuantities, Quote},
    quantities::{
        QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteDeltaLots, QuoteLots,
        QuoteLotsPerQuoteUnit, QuoteUnits,
    },
};

impl LegQuantities for Quote {
    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type DeltaLots = QuoteDeltaLots;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;
}
