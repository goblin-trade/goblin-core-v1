use core::ops::{Div, Rem};

use crate::quantities::{
    BaseAtoms, BaseAtomsDelta, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsDelta,
    BaseLotsPerBaseUnit, BaseUnits, QuantityOps, QuoteAtoms, QuoteAtomsDelta,
    QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots, QuoteLotsDelta,
    QuoteLotsPerQuoteUnit, QuoteUnits, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
};

#[derive(Default, Clone, Copy)]
pub struct Base;

#[derive(Default, Clone, Copy)]
pub struct Quote;

pub trait LegMarker {
    type Opposite: LegMarker;

    // Basic quantities
    type Lots: QuantityOps;
    type Units: QuantityOps;
    type Atoms: QuantityOps;

    // Deltas
    type LotsDelta: QuantityOps;
    // TODO trait LotsDelta * AtomsPerLot = AtomsDelta
    type AtomsDelta: QuantityOps;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit;
}

impl LegMarker for Base {
    type Opposite = Quote;

    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type LotsDelta = BaseLotsDelta;
    type AtomsDelta = BaseAtomsDelta;

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

    type LotsDelta = QuoteLotsDelta;
    type AtomsDelta = QuoteAtomsDelta;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QUOTE_ATOMS_PER_QUOTE_UNIT;
}
