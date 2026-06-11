use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        market::LotSizePair,
    },
    settlement::{sender_delta::TakeDelta, IntoUnsided, UnsidedTakeDeltaV2},
};

pub type SidedTakeDeltaV2<In> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

impl<In> IntoUnsided<In> for SidedTakeDeltaV2<In>
where
    In: LegMatcher,
{
    type Unsided = UnsidedTakeDeltaV2;

    fn into_insided(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        UnsidedTakeDeltaV2 {
            take_in: In::matching_lots_to_unsided_atoms(self.take_in, lot_size_pair),
            take_out: <<In as LegMath>::Opposite as LegMatcher>::matching_lots_to_unsided_atoms(
                self.take_out,
                lot_size_pair,
            ),
        }
    }
}
