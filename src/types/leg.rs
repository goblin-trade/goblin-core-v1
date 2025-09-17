use crate::quantities::{
    BaseAtoms, BaseAtomsPerBaseLot, BaseAtomsPerBaseUnit, BaseLots, BaseLotsPerBaseUnit, BaseUnits,
    QuantityOps, QuoteAtoms, QuoteAtomsPerQuoteLot, QuoteAtomsPerQuoteUnit, QuoteLots,
    QuoteLotsPerQuoteUnit, QuoteUnits, Ticks, BASE_ATOMS_PER_BASE_UNIT, QUOTE_ATOMS_PER_QUOTE_UNIT,
};
use core::ops::{Div, Rem};

#[derive(Default, Clone, Copy)]
pub struct Base;

#[derive(Default, Clone, Copy)]
pub struct Quote;

pub trait LegMarker {
    type Opposite: LegMarker;

    // Basic quantities
    type Lots: QuantityOps + From<u64> + PartialOrd;
    type Units: QuantityOps;
    type Atoms: QuantityOps;

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

    // Trade inputs
    const DEFAULT_PRICE_LIMIT: Ticks;

    fn price_limit_valid(_price_limit: Ticks) -> bool;
}

// Input Base = side Ask (sell)
impl LegMarker for Base {
    type Opposite = Quote;

    type Lots = BaseLots;
    type Units = BaseUnits;
    type Atoms = BaseAtoms;

    type LotsPerUnit = BaseLotsPerBaseUnit;
    type AtomsPerUnit = BaseAtomsPerBaseUnit;
    type AtomsPerLot = BaseAtomsPerBaseLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BASE_ATOMS_PER_BASE_UNIT;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;

    fn price_limit_valid(_price_limit: Ticks) -> bool {
        true
    }
}

// Input Quote = side Bid (buy)
impl LegMarker for Quote {
    type Opposite = Base;

    type Lots = QuoteLots;
    type Units = QuoteUnits;
    type Atoms = QuoteAtoms;

    type LotsPerUnit = QuoteLotsPerQuoteUnit;
    type AtomsPerUnit = QuoteAtomsPerQuoteUnit;
    type AtomsPerLot = QuoteAtomsPerQuoteLot;

    const ATOMS_PER_UNIT: Self::AtomsPerUnit = QUOTE_ATOMS_PER_QUOTE_UNIT;

    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::MAX;

    fn price_limit_valid(price_limit: Ticks) -> bool {
        price_limit > Ticks::ZERO
    }
}
