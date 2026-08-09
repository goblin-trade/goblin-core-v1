use crate::{axis::leg::leg_constants::LegConstants, settlement::ConstDefault};

pub trait LegValidator: LegConstants {
    /// Ensure that market has an integer number of atoms per lot
    ///
    /// As ATOMS_PER_UNIT is hardcoded to 10^6 for base and quote, this is effectively
    ///
    ///  **10^6 % Lot size == 0**
    ///
    /// lots_per_unit is also called lot_size
    fn lots_per_unit_valid(lots_per_unit: Self::LotsPerUnit) -> bool {
        Self::ATOMS_PER_UNIT % lots_per_unit == Self::AtomsPerUnit::DEFAULT
    }

    /// The number of atoms per lot
    ///
    /// Since we have validated the modulo invariant, this will give a whole number
    fn atoms_per_lot(lots_per_unit: Self::LotsPerUnit) -> Self::AtomsPerLot {
        Self::ATOMS_PER_UNIT / lots_per_unit
    }
}
