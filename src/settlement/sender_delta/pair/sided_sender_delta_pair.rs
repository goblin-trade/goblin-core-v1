use crate::{
    axis::leg::{leg_matcher::LegMatcher, Base, Pair, Quote},
    goblin_error::GoblinError,
    quantities::BaseLotsPerBaseUnit,
    require,
    settlement::{sender_delta::SidedSenderDeltaV2, ConstZero},
};

pub type SidedSenderDeltaPairV2 = Pair<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>>;

impl ConstZero for SidedSenderDeltaPairV2 {
    const ZEROED: Self = Pair::new(
        SidedSenderDeltaV2::<Base>::ZEROED,
        SidedSenderDeltaV2::<Quote>::ZEROED,
    );
}
