use crate::axis::leg::{LegConstants, LegValidator, Quote};

const impl LegValidator for Quote {
    fn lots_per_unit_valid(lots_per_unit: Self::LotsPerUnit) -> bool {
        Self::ATOMS_PER_UNIT.inner % lots_per_unit.inner == 0
    }

    fn atoms_per_lot(lots_per_unit: Self::LotsPerUnit) -> Self::AtomsPerLot {
        Self::AtomsPerLot::new(Self::ATOMS_PER_UNIT.inner / lots_per_unit.inner)
    }
}
