use core::ops::Rem;

use crate::quantities::{
    BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsPerBaseUnit, BaseUnits,
    QuantityOps, QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots,
    QuoteLotsPerQuoteUnit, QuoteUnits, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
};

#[derive(Clone, Copy)]
pub struct Base;

#[derive(Clone, Copy)]
pub struct Quote;

pub trait LegMarker {
    type Opposite: LegMarker;

    // Basic quantities
    type Lots: QuantityOps;
    type Units: QuantityOps;
    type Atoms: QuantityOps;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>;
    type AtomsPerLot: QuantityOps;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
}

impl LegMarker for Base {
    type Opposite = Quote;

    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BASE_ATOMS_PER_BASE_UNIT;
}

impl LegMarker for Quote {
    type Opposite = Base;

    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QUOTE_ATOMS_PER_QUOTE_UNIT;
}
