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
///! # A note on Ticks
///!
///! * Ticks use u32 while other units use u64.
///! * However the actual range of ticks is between [0, 2^21 - 1]. 21 bits are sufficient
///! to represent a tick, but we use u32 for simplicity.
///! * 16 bits are contributed by the outer index and 5 bits by the inner index.
///! * The outer index ranges from 0 to u16::MAX while the inner index ranges from 0 to 31.
///!
use crate::{define_custom_types, define_inter_type_operations};

define_custom_types!(QuoteLots<u64>, QuoteAtomsPerQuoteLot<u64>, QuoteAtoms<u64>);
define_inter_type_operations!(QuoteLots<u64>, QuoteAtomsPerQuoteLot<u64>, QuoteAtoms<u64>);

define_custom_types!(BaseLots<u64>, BaseAtomsPerBaseLot<u64>, BaseAtoms<u64>);
define_inter_type_operations!(BaseLots<u64>, BaseAtomsPerBaseLot<u64>, BaseAtoms<u64>);

define_custom_types!(
    QuoteLotsPerBaseUnitPerTick<u64>,
    Ticks<u32>,
    QuoteLotsBaseUnit<u64>
);
define_inter_type_operations!(
    QuoteLotsPerBaseUnitPerTick<u64>,
    Ticks<u32>,
    QuoteLotsBaseUnit<u64>
);

define_custom_types!(
    BaseLotsPerBaseUnit<u64>,
    QuoteLotsPerBaseUnit<u64>,
    AdjustedQuoteLots<u64>
);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_operations() {
        let lots = QuoteLots(5);
        let atoms_per_lot = QuoteAtomsPerQuoteLot(10);

        // Test multiplication
        assert_eq!(lots * atoms_per_lot, QuoteAtoms(50));
        assert_eq!(atoms_per_lot * lots, QuoteAtoms(50));

        // Test division
        let atoms = QuoteAtoms(50);
        assert_eq!(atoms / lots, QuoteAtomsPerQuoteLot(10));
        assert_eq!(atoms / atoms_per_lot, QuoteLots(5));
    }

    #[test]
    fn test_mixed_type_operations() {
        let lots_per_tick = QuoteLotsPerBaseUnitPerTick(100);
        let ticks = Ticks(5);

        // Test multiplication
        assert_eq!(lots_per_tick * ticks, QuoteLotsBaseUnit(500));
        assert_eq!(ticks * lots_per_tick, QuoteLotsBaseUnit(500));

        // Test division
        let lots = QuoteLotsBaseUnit(500);
        assert_eq!(lots / ticks, QuoteLotsPerBaseUnitPerTick(100));
        assert_eq!(lots / lots_per_tick, Ticks(5));
    }

    #[test]
    fn test_large_numbers() {
        let lots_per_tick = QuoteLotsPerBaseUnitPerTick(1_000_000);
        let ticks = Ticks(1_000);

        // Should handle larger numbers without overflow since result type is u64
        assert_eq!(lots_per_tick * ticks, QuoteLotsBaseUnit(1_000_000_000));
    }
}
