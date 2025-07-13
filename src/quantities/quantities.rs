///! This module defines custom types for quantities used in the exchange.
///!
///! # Quantities and equations are
///!
///! 1. QuoteLots * QuoteAtomsPerQuoteLot = QuoteAtoms
///! 2. BaseLots * BaseAtomsPerBaseLot = BaseAtoms
///! 3. QuoteLotsPerBaseUnitPerTick * Ticks = QuoteLotsBaseUnit
///! 4. QuoteLots * BaseLotsPerBaseUnit = AdjustedQuoteLots
///! 5. QuoteLotsPerBaseUnit * BaseLots = AdjustedQuoteLots
///!
///! # Direct and intermediate units
///!
///! * 'lots per unit' is used for calculations. 'atoms per lot' is an intermediate
///! unit that can be avoided.
///!
///! * Since every token is adjusted to 6 decimal places,  atoms_per_unit = 10^6
///! Therefore lots_per_unit = atoms_per_unit / atoms_per_lot = 10^6 / atoms_per_lot
///!
///! * Direct units- BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, QuoteLotsPerBaseUnitPerTick
///! * Indirect units- BaseAtomsPerBaseLot, QuoteAtomsPerQuoteLot, QuoteAtomsPerBaseUnitPerTick
///!
///! # A note on Ticks
///!
///! * Ticks use u32 while other units use u64.
///! * However the actual range of ticks is between [0, 2^21 - 1]. 21 bits are sufficient
///! to represent a tick, but we use u32 for simplicity.
///! * 16 bits are contributed by the outer index and 5 bits by the inner index.
///! * The outer index ranges from 0 to u16::MAX while the inner index ranges from 0 to 31.
use crate::{allow_mod, define_custom_types, define_inter_type_operations};

define_custom_types!(QuoteLots<u64>, QuoteAtoms<u64>);
// define_custom_types!(QuoteLots<u64>, QuoteAtomsPerQuoteLot<u64>, QuoteAtoms<u64>);
// define_inter_type_operations!(QuoteLots<u64>, QuoteAtomsPerQuoteLot<u64>, QuoteAtoms<u64>);

define_custom_types!(BaseLots<u64>, BaseAtoms<u64>);
// define_custom_types!(BaseLots<u64>, BaseAtomsPerBaseLot<u64>, BaseAtoms<u64>);
// define_inter_type_operations!(BaseLots<u64>, BaseAtomsPerBaseLot<u64>, BaseAtoms<u64>);

// The number of lots per unit
//
// Since one unit has 10^6 atoms, the legal values are
// * MAX: 10^6 lots per unit, i.e. 1 lot = 1 atom, i.e. 1 lot = 1 / 10^6 unit
// * MIN: 1 lot per unit, i.e. 11 lot = 10^6 atom, i.e. 1 lot = 1 unit
//
// A value if legal iff 10^6 % lots per unit == 0
define_custom_types!(BaseLotsPerBaseUnit<u64>, QuoteLotsPerQuoteUnit<u64>);

define_custom_types!(
    QuoteLotsPerBaseUnitPerTick<u64>,
    Ticks<u32>,
    QuoteLotsPerBaseUnit<u64>
);
define_inter_type_operations!(
    QuoteLotsPerBaseUnitPerTick<u64>,
    Ticks<u32>,
    QuoteLotsPerBaseUnit<u64>
);

define_custom_types!(AdjustedQuoteLots<u64>);

define_inter_type_operations!(
    QuoteLots<u64>,
    BaseLotsPerBaseUnit<u64>,
    AdjustedQuoteLots<u64>
);
define_inter_type_operations!(
    QuoteLotsPerBaseUnit<u64>,
    BaseLots<u64>,
    AdjustedQuoteLots<u64>
);

allow_mod!(QuoteLotsPerBaseUnitPerTick, BaseLotsPerBaseUnit);

/// Token amounts are normalized to 10^6 atoms per unit.
const ATOMS_PER_UNIT: u64 = 1_000_000;

impl BaseLotsPerBaseUnit {
    pub fn valid(&self) -> bool {
        ATOMS_PER_UNIT % self.0 == 0
    }
}

impl QuoteLotsPerQuoteUnit {
    pub fn valid(&self) -> bool {
        ATOMS_PER_UNIT % self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_type_operations() {
        let lots_per_tick = QuoteLotsPerBaseUnitPerTick(100);
        let ticks = Ticks(5);

        // Test multiplication
        assert_eq!(lots_per_tick * ticks, QuoteLotsPerBaseUnit(500));
        assert_eq!(ticks * lots_per_tick, QuoteLotsPerBaseUnit(500));

        // Test division
        let lots = QuoteLotsPerBaseUnit(500);
        assert_eq!(lots / ticks, QuoteLotsPerBaseUnitPerTick(100));
        assert_eq!(lots / lots_per_tick, Ticks(5));
    }

    #[test]
    fn test_large_numbers() {
        let lots_per_tick = QuoteLotsPerBaseUnitPerTick(1_000_000);
        let ticks = Ticks(1_000);

        // Should handle larger numbers without overflow since result type is u64
        assert_eq!(lots_per_tick * ticks, QuoteLotsPerBaseUnit(1_000_000_000));
    }
}
