use crate::{
    axis::leg::{Quote, leg_quantities::LegQuantities},
    quantities::{
        QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots,
        QuoteLotsPerQuoteUnit, QuoteUnits,
    },
};

impl LegQuantities for Quote {
    type Lots = QuoteLots<u64>;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms<u64>;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;
}
