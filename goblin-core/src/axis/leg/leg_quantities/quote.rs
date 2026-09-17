use crate::{
    axis::leg::{Quote, leg_quantities::LegQuantities},
    quantities::{
        QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots,
        QuoteLotsPerQuoteUnit, QuoteUnits,
    },
};

impl LegQuantities for Quote {
    type Lots = QuoteLots<u64>;
    type Units = QuoteUnits<u64>;
    type Atoms = QuoteAtoms<u64>;

    type LotsPerUnit = QuoteLotsPerQuoteUnit<u64>;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit<u64>;
    type AtomsPerLot = QuoteAtomsPerQuoteLot<u64>;
}
