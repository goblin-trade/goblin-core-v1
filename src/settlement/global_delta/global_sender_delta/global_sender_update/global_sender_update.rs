use crate::{
    markets::LotSizePair,
    settlement::local_delta::TakerDelta,
    types::{Base, LegMarker, Pair, PairAccessor, Quote},
};

/// A balance update in the global token level namespace
///
/// Unlike TakerDelta which holds updates for tokens on both side,
/// GlobalUpdate represents balance updates for one token.
#[derive(Default, PartialEq)]
pub struct GlobalSenderUpdate<In: LegMarker> {
    /// Atoms traded in by taker
    pub taker_in: In::Atoms,

    /// Atoms obtained by taker
    pub taker_out: In::Atoms,

    /// Atoms released due to taker self-trading
    pub taker_self_trade_unlocked: In::Atoms,
}

impl<In> GlobalSenderUpdate<In>
where
    In: LegMarker
        + PairAccessor<
            <Base as LegMarker>::LotsPerUnit,
            <Quote as LegMarker>::LotsPerUnit,
            Result = In::LotsPerUnit,
        > + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
    In::Opposite:
        PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In::Opposite>>,
{
    pub fn new(
        taker_delta_pair: &Pair<TakerDelta<Base>, TakerDelta<Quote>>,
        lot_size_pair: &LotSizePair,
    ) -> Self {
        let base_lot_size = lot_size_pair.base;
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let delta = In::get_leg(taker_delta_pair);
        let delta_opposite = In::Opposite::get_leg(taker_delta_pair);

        let taker_in = In::matching_lots_to_atoms(delta.taker_in, base_lot_size, atoms_per_lot);

        let taker_out =
            In::matching_lots_to_atoms(delta_opposite.taker_out, base_lot_size, atoms_per_lot);

        let taker_self_trade_unlocked = In::matching_lots_to_atoms(
            delta_opposite.taker_self_trade_unlocked,
            base_lot_size,
            atoms_per_lot,
        );

        Self {
            taker_in,
            taker_out,
            taker_self_trade_unlocked,
        }
    }
}
