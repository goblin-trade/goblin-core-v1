use crate::{
    markets::LotSizePair,
    settlement::local_delta::{MakerDelta, MakerDeltaPair},
    types::{Base, LegMarker, PairAccessor, Quote},
};

/// Pending update to maker state for the token at In
/// Eg. for In: Base, these updates will apply on the Base token maker delta
pub struct GlobalMakerUpdate<In: LegMarker> {
    /// Atoms traded in by taker and gained by maker
    pub taker_in: In::Atoms,

    /// Atoms obtained by taker and lost by maker
    pub taker_out: In::Atoms,
}

impl<In> GlobalMakerUpdate<In>
where
    In: LegMarker
        + PairAccessor<
            <Base as LegMarker>::LotsPerUnit,
            <Quote as LegMarker>::LotsPerUnit,
            Result = In::LotsPerUnit,
        > + PairAccessor<MakerDelta<Base>, MakerDelta<Quote>, Result = MakerDelta<In>>,
    In::Opposite:
        PairAccessor<MakerDelta<Base>, MakerDelta<Quote>, Result = MakerDelta<In::Opposite>>,
{
    pub fn new(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        let base_lot_size = lot_size_pair.base;
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let delta = In::get_leg(maker_delta_pair);
        let delta_opposite = In::Opposite::get_leg(maker_delta_pair);

        let taker_in = In::matching_lots_to_atoms(delta.taker_in, base_lot_size, atoms_per_lot);

        let taker_out =
            In::matching_lots_to_atoms(delta_opposite.taker_out, base_lot_size, atoms_per_lot);

        Self {
            taker_in,
            taker_out,
        }
    }
}
