use crate::{define_legged_type, types::leg::LegMarker};

// Basic quantity types with leg markers
define_legged_type!(AtomsV2<u64>);
define_legged_type!(LotsV2<u64>);

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
