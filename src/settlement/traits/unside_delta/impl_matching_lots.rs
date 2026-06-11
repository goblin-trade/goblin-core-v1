use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::LotSizePair,
    },
    quantities::{UnsideQuantity, UnsidedAtoms},
    settlement::UnsideDelta,
    types::StoreReader,
};

impl<In> UnsideDelta<In> for In::MatchingLots
where
    In: LegMatcher,
{
    type Unsided = UnsidedAtoms;

    fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        let lot_size = In::get(lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let base_lot_size = Base::get(lot_size_pair);

        let lots = In::decode_matching_lots(*self, base_lot_size);
        let atoms = lots * atoms_per_lot;

        atoms.unsided()
    }
}
