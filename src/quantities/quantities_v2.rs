use crate::{define_dimensionless_type, define_legged_type, types::leg::LegMarker};

// Basic quantity types with leg markers
define_legged_type!(AtomsV2<u64>);
define_legged_type!(LotsV2<u64>);

// The only dimensionless unit
define_dimensionless_type!(TicksV2<u32>);

// TODO composite units

// How to handle
// - legless unit: Ticks
// - complex units with multiplication and division?
#[cfg(test)]
mod tests {
    use crate::types::{Base, Quote};

    use super::*;

    #[test]
    fn test_addition() {
        let atoms_base_0 = AtomsV2::<Base>::new(0);
        let atoms_base_1 = AtomsV2::<Base>::new(1);

        let atoms_quote_0 = AtomsV2::<Quote>::new(1);

        let total_base = atoms_base_0 + atoms_base_1;

        // let gg = atoms_quote_0 + atoms_base_0;
    }
}
