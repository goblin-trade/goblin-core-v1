use crate::{
    markets::LotSizePair,
    settlement::{local_delta::MakerDeltaPair, MatchedAtoms, MatchedLots, MatchedLotsPair},
    types::{Base, LegMarker, PairAccessor, Quote},
};

/// Pending update to maker state for the token at In
/// Eg. for In: Base, these updates will apply on the Base token maker delta
pub struct GlobalMakerUpdate<In: LegMarker> {
    /// Matched atoms
    pub matched_atoms: MatchedAtoms<In>,
}

impl<In> GlobalMakerUpdate<In>
where
    In: LegMarker
        + PairAccessor<
            <Base as LegMarker>::LotsPerUnit,
            <Quote as LegMarker>::LotsPerUnit,
            Result = In::LotsPerUnit,
        > + PairAccessor<MatchedLots<Base>, MatchedLots<Quote>, Result = MatchedLots<In>>,
    In::Opposite:
        PairAccessor<MatchedLots<Base>, MatchedLots<Quote>, Result = MatchedLots<In::Opposite>>,
{
    pub fn new(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        let matched_lots_pair = MatchedLotsPair::from(maker_delta_pair);
        let matched_atoms = MatchedAtoms::new(&matched_lots_pair, lot_size_pair);

        Self { matched_atoms }
    }
}
