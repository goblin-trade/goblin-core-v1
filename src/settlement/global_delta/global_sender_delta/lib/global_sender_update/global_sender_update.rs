use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities, Base, Leg, Pair, Quote},
        market::LotSizePair,
    },
    settlement::{MatchedAtoms, MatchedLots},
    types::{StoreReader, Tuple},
};

/// A balance update in the global token level namespace
///
/// Unlike TakerDelta which holds updates for tokens on both side,
/// GlobalUpdate represents balance updates for one token.
#[derive(Default, PartialEq, Clone, Copy)]
pub struct GlobalSenderUpdate<In: LegMatcher> {
    /// Matched atoms for the given token
    pub matched_atoms: MatchedAtoms<In>,
}

impl<In> GlobalSenderUpdate<In>
where
    In: LegMatcher
        + StoreReader<
            Tuple<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit, Leg>,
            Result = In::LotsPerUnit,
        > + StoreReader<Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>, Result = MatchedLots<In>>,
    In::Opposite: StoreReader<
        Tuple<MatchedLots<Base>, MatchedLots<Quote>, Leg>,
        Result = MatchedLots<In::Opposite>,
    >,
{
    pub fn new(
        taker_delta_pair: &Pair<MatchedLots<Base>, MatchedLots<Quote>>,
        lot_size_pair: &LotSizePair,
    ) -> Self {
        let matched_atoms = MatchedAtoms::new(taker_delta_pair, lot_size_pair);
        Self { matched_atoms }
    }
}
