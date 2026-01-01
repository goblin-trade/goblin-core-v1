use crate::{
    market::LotSizePair,
    settlement::{
        local_delta::{TakerDelta, TakerDeltaPair},
        MatchedAtoms, MatchedLots, MatchedLotsPair,
    },
    types::{Base, LegMarker, LegQuantities, Quote, TupleReader},
};

/// A balance update in the global token level namespace
///
/// Unlike TakerDelta which holds updates for tokens on both side,
/// GlobalUpdate represents balance updates for one token.
#[derive(Default, PartialEq, Clone, Copy)]
pub struct GlobalSenderUpdate<In: LegMarker> {
    /// Matched atoms for the given token
    pub matched_atoms: MatchedAtoms<In>,

    /// Atoms released due to taker self-trading
    pub taker_self_trade_unlocked: In::Atoms,
}

impl<In> GlobalSenderUpdate<In>
where
    In: LegMarker
        + TupleReader<
            <Base as LegQuantities>::LotsPerUnit,
            <Quote as LegQuantities>::LotsPerUnit,
            (Base, Quote),
            Result = In::LotsPerUnit,
        > + TupleReader<MatchedLots<Base>, MatchedLots<Quote>, (Base, Quote), Result = MatchedLots<In>>,
    In::Opposite: TupleReader<
            MatchedLots<Base>,
            MatchedLots<Quote>,
            (Base, Quote),
            Result = MatchedLots<In::Opposite>,
        > + TupleReader<
            TakerDelta<Base>,
            TakerDelta<Quote>,
            (Base, Quote),
            Result = TakerDelta<In::Opposite>,
        >,
{
    pub fn new(taker_delta_pair: &TakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        let matched_lots_pair = MatchedLotsPair::from(taker_delta_pair);
        let matched_atoms = MatchedAtoms::new(&matched_lots_pair, lot_size_pair);

        let base_lot_size = Base::get(lot_size_pair);
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let delta_opposite = In::Opposite::get_leg(taker_delta_pair);
        let taker_self_trade_unlocked = In::matching_lots_to_atoms(
            delta_opposite.taker_self_trade_unlocked,
            base_lot_size,
            atoms_per_lot,
        );

        Self {
            matched_atoms,
            taker_self_trade_unlocked,
        }
    }
}
