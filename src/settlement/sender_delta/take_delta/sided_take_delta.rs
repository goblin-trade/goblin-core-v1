use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_math::LegMath},
        market::LotSizePair,
    },
    settlement::{sender_delta::TakeDelta, UnsideDelta, UnsidedTakeDeltaV2},
};

pub type SidedTakeDeltaV2<In> =
    TakeDelta<<In as LegMath>::MatchingLots, <<In as LegMath>::Opposite as LegMath>::MatchingLots>;

// impl<In> UnsideDelta<In> for SidedTakeDeltaV2<In>
// where
//     In: LegMatcher,
// {
//     type Unsided = UnsidedTakeDeltaV2;

//     fn unside(&self, lot_size_pair: &LotSizePair) -> Self::Unsided {
//         todo!()
//         // UnsidedTakeDeltaV2 {
//         //     take_in: self.take_in.unside(lot_size_pair),
//         //     take_out: self.take_out.unside(lot_size_pair),
//         // }
//     }
// }
