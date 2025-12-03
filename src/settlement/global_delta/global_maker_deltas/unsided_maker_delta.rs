use crate::quantities::{QuantityOps, UnsidedAtoms};

/// Maker delta for a token
///
/// Unlike `MakerDelta` which is namespaced by market and tracks state of two tokens,
/// this delta tracks updates for a single token.
#[derive(Clone, Copy)]
pub struct UnsidedMakerDelta {
    pub free_atoms_in: UnsidedAtoms,
    pub locked_atoms_out: UnsidedAtoms,
}

impl UnsidedMakerDelta {
    pub const fn new() -> Self {
        Self {
            free_atoms_in: UnsidedAtoms::ZERO,
            locked_atoms_out: UnsidedAtoms::ZERO,
        }
    }
}

// #[derive(Default)]
// pub struct MakerUpdate {
//     pub free_atoms_in: UnsidedAtoms,
//     pub locked_atoms_out: UnsidedAtoms,
// }

// impl MakerUpdate {
//     fn new<In>(
//         maker_delta: &MakerDelta,
//         lot_size_pair: LotSizePair,
//         base_lot_size: BaseLotsPerBaseUnit,
//     ) -> Self
//     where
//         In: LegMarker
//             + PairAccessor<
//                 <Base as LegMarker>::LotsPerUnit,
//                 <Quote as LegMarker>::LotsPerUnit,
//                 Result = In::LotsPerUnit,
//             > + PairAccessor<MakerSideDelta<Base>, MakerSideDelta<Quote>, Result = MakerSideDelta<In>>,
//         In::Opposite: PairAccessor<
//             MakerSideDelta<Base>,
//             MakerSideDelta<Quote>,
//             Result = MakerSideDelta<In::Opposite>,
//         >,
//     {
//         let lot_size = *In::get_leg(&lot_size_pair);
//         let atoms_per_lot = In::atoms_per_lot(lot_size);

//         let delta = In::get_leg(maker_delta);
//         let delta_opposite = In::Opposite::get_leg(maker_delta);

//         let free_atoms_in = In::matching_lots_to_atoms_unsided(
//             delta.free_matching_lots_in,
//             base_lot_size,
//             atoms_per_lot,
//         );

//         let locked_atoms_out = In::matching_lots_to_atoms_unsided(
//             delta_opposite.locked_matching_lots_out,
//             base_lot_size,
//             atoms_per_lot,
//         );

//         Self {
//             locked_atoms_out,
//             free_atoms_in,
//         }
//     }
// }
