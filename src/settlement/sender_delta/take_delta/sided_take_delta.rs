use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        market::LotSizePair,
    },
    settlement::{sender_delta::TakeDelta, unside_matching_lots, UnsideDelta, UnsidedTakeDeltaV2},
};

pub type SidedTakeDeltaV2<In> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

impl<In> UnsideDelta<In> for SidedTakeDeltaV2<In>
where
    In: LegMatcher,
{
    type Unsided = UnsidedTakeDeltaV2;

    fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        UnsidedTakeDeltaV2 {
            take_in: unside_matching_lots::<In>(self.take_in, lot_size_pair),
            take_out: unside_matching_lots::<In::Opposite>(self.take_out, lot_size_pair),
        }
    }
}
