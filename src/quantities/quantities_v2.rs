///! This module defines custom types for quantities using LegMarker for cleaner type signatures.
///!
///! # Migration from v1 to v2
///!
///! * BaseLots -> Lots<Base>
///! * QuoteLots -> Lots<Quote>
///! * BaseAtoms -> Atoms<Base>
///! * QuoteAtoms -> Atoms<Quote>
///! * BaseLotsPerBaseUnit -> LotsPerUnit<Base>
///! * QuoteLotsPerQuoteUnit -> LotsPerUnit<Quote>
///!
///! # Quantities and equations remain the same
///!
///! 1. Lots<Quote> * AtomsPerLot<Quote> = Atoms<Quote>
///! 2. Lots<Base> * AtomsPerLot<Base> = Atoms<Base>
///! 3. LotsPerUnitPerTick<Quote, Base> * Ticks = LotsPerUnit<Quote, Base>
///! 4. Lots<Quote> * LotsPerUnit<Base> = AdjustedLots<Quote>
///! 5. LotsPerUnit<Quote, Base> * Lots<Base> = AdjustedLots<Quote>
///!
///! # Benefits of LegMarker approach
///!
///! * Type safety: Cannot accidentally mix base and quote operations
///! * Cleaner API: Lots<Base> vs BaseLots
///! * Generic programming: Can write functions that work with either leg
///! * Future extensibility: Easy to add new leg types if needed
use crate::{
    define_custom_types_v2,
    quantities::Ticks,
    types::leg::{Base, LegMarker, Quote},
};

// Basic quantity types with leg markers
define_custom_types_v2!(LotsV2<L: LegMarker, u64>, AtomsV2<L: LegMarker, u64>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let atoms_base_0 = AtomsV2::<Base>::new(0);
        let atoms_base_1 = AtomsV2::<Base>::new(1);

        let atoms_quote_0 = AtomsV2::<Quote>::new(1);

        let total_base = atoms_base_0 + atoms_base_1;
    }
}
