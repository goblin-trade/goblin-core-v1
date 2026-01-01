use crate::{
    quantities::{
        AsUnsided, BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots,
        BaseLotsPerBaseUnit, BaseUnits, QuantityOps, QuoteAtoms, QuoteAtomsPerQuoteLot,
        QuoteAtomsPerQuoteUnit, QuoteLots, QuoteLotsPerQuoteUnit, QuoteUnits, P1, Z0,
    },
    types::{Base, Quote},
};
use core::ops::{Div, Mul, Rem};

pub trait LegQuantities: Sized {
    // Basic quantities
    type Lots: QuantityOps + From<u64> + PartialOrd + Mul<Self::AtomsPerLot, Output = Self::Atoms>;
    type Units: QuantityOps;
    type Atoms: QuantityOps + AsUnsided<Self, Z0, Z0, P1>;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps;
}

impl LegQuantities for Base {
    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;
}

impl LegQuantities for Quote {
    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;
}
