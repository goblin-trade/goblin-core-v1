use crate::quantities::{
    BaseAtoms, BaseAtomsDelta, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsDelta,
    BaseLotsPerBaseUnit, BaseUnits, QuantityOps, QuoteAtoms, QuoteAtomsDelta,
    QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots, QuoteLotsDelta,
    QuoteLotsPerQuoteUnit, QuoteUnits, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
};
use core::ops::{Div, Mul, Rem};

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
    type LotsDelta: QuantityOps + Mul<Self::AtomsPerLot, Output = Self::AtomsDelta>;

    // TODO trait to convert AtomsDelta to the original AtomsDelta
    // Fix duplicate naming between LegMarker::AtomsDelta and the original legless AtomsDelta
    // The legless delta is an accumulator used for settlement
    //
    // TODO what if we avoid signed deltas in the market namespace, similar to the Maker accumulators?
    // Track locked, unlocked, consumed lots
    // When we leave market namespace, convert lots to atoms and net them into a delta
    //
    // This will get rid of both LotsDelta and AtomsDelta
    type AtomsDelta: QuantityOps;

    // Ratios
    type LotsPerUnit: QuantityOps;
    type AtomsPerUnit: QuantityOps
        + Rem<Self::LotsPerUnit, Output = Self::AtomsPerUnit>
        + Div<Self::LotsPerUnit, Output = Self::AtomsPerLot>;
    type AtomsPerLot: QuantityOps;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit;

    /// Ensure that market has an integer number of atoms per lot
    ///
    /// As ATOMS_PER_UNIT is hardcoded to 10^6 for base and quote, this is effectively
    ///
    ///  **10^6 % Lot size == 0**
    ///
    /// lots_per_unit is also called lot_size
    fn lots_per_unit_valid(lots_per_unit: Self::LotsPerUnit) -> bool {
        Self::ATOMS_PER_UNIT % lots_per_unit == Self::AtomsPerUnit::ZERO
    }

    /// The number of atoms per lot
    ///
    /// Since we have validated the modulo invariant, this will give a whole number
    fn atoms_per_lot(lots_per_unit: Self::LotsPerUnit) -> Self::AtomsPerLot {
        Self::ATOMS_PER_UNIT / lots_per_unit
    }
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
