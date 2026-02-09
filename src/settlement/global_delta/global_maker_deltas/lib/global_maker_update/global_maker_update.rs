use crate::{
    market::LotSizePair,
    settlement::{local_delta::MakerDeltaPair, MatchedAtoms, MatchedLots, MatchedLotsPair},
    types::{Base, Leg, LegMatcher, LegQuantities, LegValidator, Quote, StoreReader, Tuple},
};

/// Pending update to maker state for the token at In
/// Eg. for In: Base, these updates will apply on the Base token maker delta
#[derive(Clone, Copy)]
pub struct GlobalMakerUpdate<In: LegMatcher> {
    /// Matched atoms
    pub matched_atoms: MatchedAtoms<In>,
}

impl<In> GlobalMakerUpdate<In>
where
    In: LegMatcher
        + LegValidator
        + StoreReader<
            Tuple<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit, Leg>,
            Result = In::LotsPerUnit,
        > + StoreReader<Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>, Result = MatchedLots<In>>,
    In::Opposite: StoreReader<
        Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>,
        Result = MatchedLots<In::Opposite>,
    >,
{
    pub fn new(maker_delta_pair: &MakerDeltaPair, lot_size_pair: &LotSizePair) -> Self {
        let matched_lots_pair = MatchedLotsPair::from(maker_delta_pair);
        let matched_atoms = MatchedAtoms::new(&matched_lots_pair, lot_size_pair);

        Self { matched_atoms }
    }
}
