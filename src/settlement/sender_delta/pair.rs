use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::{
        sender_delta::{alias::SidedSenderDeltaV2, SidedTakeDeltaV2},
        ConstZero,
    },
};

pub type SidedSenderDeltaPairV2 = Pair<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>>;
pub type SidedTakeDeltaPairV2 = Pair<SidedTakeDeltaV2<Base>, SidedTakeDeltaV2<Quote>>;

impl ConstZero for SidedSenderDeltaPairV2 {
    const ZEROED: Self = Pair::new(
        SidedSenderDeltaV2::<Base>::ZEROED,
        SidedSenderDeltaV2::<Quote>::ZEROED,
    );
}

impl ConstZero for SidedTakeDeltaPairV2 {
    const ZEROED: Self = Pair::new(
        SidedTakeDeltaV2::<Base>::ZEROED,
        SidedTakeDeltaV2::<Quote>::ZEROED,
    );
}
