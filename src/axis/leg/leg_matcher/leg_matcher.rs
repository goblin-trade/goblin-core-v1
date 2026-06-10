use crate::{
    axis::{
        leg::{
            leg_constants::LegConstants, leg_coordinates::LegCoordinates,
            leg_iterator::LegIterator, leg_math::LegMath, leg_quantities::LegQuantities,
            leg_reader::LegReader, leg_validator::LegValidator, Base, Quote,
        },
        market::LotSizePair,
    },
    quantities::{AsUnsided, UnsidedAtoms},
    types::StoreReader,
};

/// Supertrait for leg operations
pub trait LegMatcher:
    LegQuantities + LegMath + LegConstants + LegValidator + LegCoordinates + LegIterator + LegReader
{
    fn matching_lots_to_unsided_atoms(
        matching_lots: Self::MatchingLots,
        lot_size_pair: &LotSizePair,
    ) -> UnsidedAtoms {
        let lot_size = Self::get(lot_size_pair);
        let atoms_per_lot = Self::atoms_per_lot(lot_size);

        let base_lot_size = Base::get(lot_size_pair);
        Self::matching_lots_to_atoms(matching_lots, base_lot_size, atoms_per_lot).unsided()
    }
}

impl LegMatcher for Base {}
impl LegMatcher for Quote {}
