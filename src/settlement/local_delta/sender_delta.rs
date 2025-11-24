use core::marker::PhantomData;

use crate::{
    markets::LotSizePair,
    matching::MatchResult,
    quantities::UnsidedAtoms,
    types::{Base, LegMarker, Pair, PairAccessor, Quote},
};

/// The sender delta of local namespace
///
/// Values are denominated in MatchingLots. Convert it to TakerTokenUpdate
/// so it can be added to the global delta
#[derive(Default)]
pub struct SenderDelta {
    /// The results of matching take orders
    pub take_result_pair: Pair<MatchResult<Base>, MatchResult<Quote>>,
}

// Pending updates for the taker per token after performing
// taker ask and quote trades on a market
pub struct SenderGlobalUpdate<In: LegMarker> {
    pub free_atoms_in: UnsidedAtoms,
    pub locked_atoms_out: UnsidedAtoms,
    pub atoms_released_by_self_trade: UnsidedAtoms,
    _marker: core::marker::PhantomData<In>,
}

impl SenderDelta {
    pub fn to_global_update<In>(&self, lot_size_pair: LotSizePair) -> SenderGlobalUpdate<In>
    where
        In: LegMarker
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In>>,
        In::Opposite:
            PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In::Opposite>>,
    {
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let delta = In::get_leg(&self.take_result_pair);
        let delta_opposite = In::Opposite::get_leg(&self.take_result_pair);

        let free_atoms_in = In::matching_lots_to_atoms_unsided(
            delta.free_matching_lots_in,
            lot_size_pair.base,
            atoms_per_lot,
        );

        let locked_atoms_out = In::matching_lots_to_atoms_unsided(
            delta_opposite.locked_matching_lots_out,
            lot_size_pair.base,
            atoms_per_lot,
        );

        let atoms_released_by_self_trade = In::matching_lots_to_atoms_unsided(
            delta_opposite.released_by_self_trade,
            lot_size_pair.base,
            atoms_per_lot,
        );

        SenderGlobalUpdate::<In> {
            free_atoms_in,
            locked_atoms_out,
            atoms_released_by_self_trade,
            _marker: PhantomData,
        }
    }
}
