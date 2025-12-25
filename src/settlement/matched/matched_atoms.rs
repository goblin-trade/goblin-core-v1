use crate::{
    market::LotSizePair,
    settlement::{MatchedLots, MatchedLotsPair},
    types::{Base, LegMarker, Quote, TupleReader},
};

/// Matched atoms for a given token
#[derive(Default, PartialEq, Clone, Copy)]
pub struct MatchedAtoms<In: LegMarker> {
    /// Atoms traded in by taker and gained by maker
    pub taker_in: In::Atoms,

    /// Atoms obtained by taker and lost by maker
    pub taker_out: In::Atoms,
}

impl<In> MatchedAtoms<In>
where
    In: LegMarker
        + TupleReader<
            <Base as LegMarker>::LotsPerUnit,
            <Quote as LegMarker>::LotsPerUnit,
            (Base, Quote),
            Result = In::LotsPerUnit,
        > + TupleReader<MatchedLots<Base>, MatchedLots<Quote>, (Base, Quote), Result = MatchedLots<In>>,
    In::Opposite: TupleReader<
        MatchedLots<Base>,
        MatchedLots<Quote>,
        (Base, Quote),
        Result = MatchedLots<In::Opposite>,
    >,
{
    pub fn new(matched_lots_pair: &MatchedLotsPair, lot_size_pair: &LotSizePair) -> Self {
        let base_lot_size = Base::get(&lot_size_pair);
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let matched_lots = In::get_leg(matched_lots_pair);
        let matched_lots_opposite = In::Opposite::get_leg(matched_lots_pair);

        let taker_in =
            In::matching_lots_to_atoms(matched_lots.taker_in, base_lot_size, atoms_per_lot);

        let taker_out = In::matching_lots_to_atoms(
            matched_lots_opposite.taker_out,
            base_lot_size,
            atoms_per_lot,
        );

        Self {
            taker_in,
            taker_out,
        }
    }
}
