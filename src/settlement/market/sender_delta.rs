use crate::{
    markets::LotSizePair,
    matching::MatchResult,
    quantities::{BaseLotsPerBaseUnit, UnsidedAtoms},
    types::{Base, LegMarker, Pair, PairAccessor, Quote},
};

// The sender delta currently contains results of matching base in and quote in
// take orders. TODO add other types in here later for limit orders and cancellations.
pub type SenderDelta = Pair<MatchResult<Base>, MatchResult<Quote>>;

// // Pending updates for the taker per token after performing
// // taker ask and quote trades on a market
// pub struct TakerTokenUpdate {
//     pub free_atoms_in: UnsidedAtoms,
//     pub locked_atoms_out: UnsidedAtoms,
//     pub atoms_released_by_self_trade: UnsidedAtoms,
// }

// impl SenderDelta {
//     pub fn to_taker_token_update<In>(
//         &self,
//         lot_size_pair: LotSizePair,
//         base_lot_size: BaseLotsPerBaseUnit,
//     ) -> TakerTokenUpdate
//     where
//         In: LegMarker
//             + PairAccessor<
//                 <Base as LegMarker>::LotsPerUnit,
//                 <Quote as LegMarker>::LotsPerUnit,
//                 Result = In::LotsPerUnit,
//             > + PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In>>,
//         In::Opposite:
//             PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In::Opposite>>,
//     {
//         let lot_size = *In::get_leg(&lot_size_pair);
//         let atoms_per_lot = In::atoms_per_lot(lot_size);

//         let delta = In::get_leg(self);
//         let delta_opposite = In::Opposite::get_leg(self);

//         let free_atoms_in = In::matching_lots_to_atoms_unsided(
//             delta.maker_side_delta.free_matching_lots_in,
//             base_lot_size,
//             atoms_per_lot,
//         );

//         let locked_atoms_out = In::matching_lots_to_atoms_unsided(
//             delta_opposite.maker_side_delta.locked_matching_lots_out,
//             base_lot_size,
//             atoms_per_lot,
//         );

//         let atoms_released_by_self_trade = In::matching_lots_to_atoms_unsided(
//             delta_opposite.released_by_self_trade,
//             base_lot_size,
//             atoms_per_lot,
//         );

//         TakerTokenUpdate {
//             free_atoms_in,
//             locked_atoms_out,
//             atoms_released_by_self_trade,
//         }
//     }
// }
