use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        market::LotSizePair,
    },
    settlement::{unside_matching_lots, MakeDelta, UnsideDelta, UnsidedMakeDeltaV2},
};

pub type SidedMakeDeltaV2<In> = MakeDelta<<<In as LegMath>::Opposite as LegMath>::MatchingLots>;

impl<In> UnsideDelta<In> for SidedMakeDeltaV2<In>
where
    In: LegMatcher,
{
    type Unsided = UnsidedMakeDeltaV2;

    fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
        UnsidedMakeDeltaV2::new(
            unside_matching_lots::<In::Opposite>(self.0, lot_size_pair),
            unside_matching_lots::<In::Opposite>(self.1, lot_size_pair),
        )
    }
}
