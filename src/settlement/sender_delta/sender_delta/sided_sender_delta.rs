use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        market::LotSizePair,
    },
    settlement::{
        SenderDelta, SidedMakeDeltaV2, SidedTakeDeltaV2, UnsideDelta, UnsidedMakeDeltaV2,
        UnsidedSenderDeltaV2, UnsidedTakeDeltaV2,
    },
};

pub type SidedSenderDeltaV2<In> = SenderDelta<
    <In as LegMath>::MatchingLots,
    <<In as LegMath>::Opposite as LegMath>::MatchingLots,
>;

impl<In> UnsideDelta<In> for SidedSenderDeltaV2<In>
where
    In: LegMatcher,
    SidedTakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedTakeDeltaV2>,
    SidedMakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedMakeDeltaV2>,
{
    type Unsided = UnsidedSenderDeltaV2;

    fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        let take = self.take.unside(lot_size_pair);
        let make = self.make.unside(lot_size_pair);

        UnsidedSenderDeltaV2 { take, make }
    }
}
