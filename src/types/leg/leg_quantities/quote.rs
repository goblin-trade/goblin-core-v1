use crate::{
    quantities::{
        QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots,
        QuoteLotsPerQuoteUnit, QuoteUnits,
    },
    types::{LegQuantities, Quote},
};

impl LegQuantities for Quote {
    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;
}
