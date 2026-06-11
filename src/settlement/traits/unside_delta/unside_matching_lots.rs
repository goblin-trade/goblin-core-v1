use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::LotSizePair,
    },
    quantities::{UnsideQuantity, UnsidedAtoms},
    types::StoreReader,
};

/// Cast matching lots into UnsidedAtoms
///
/// We cannot use the UnsideDelta trait on In::MatchingLots because this is
/// an opaque type whose solution space can overlap with other structs.
pub fn unside_matching_lots<In: LegMatcher>(
    matching_lots: In::MatchingLots,
    lot_size_pair: &LotSizePair,
) -> UnsidedAtoms {
    let base_lot_size = Base::get(lot_size_pair);
    let lot_size = In::get(lot_size_pair);
    let atoms_per_lot = In::atoms_per_lot(lot_size);

    let lots = In::decode_matching_lots(matching_lots, base_lot_size);
    let atoms = lots * atoms_per_lot;

    atoms.unsided()
}
