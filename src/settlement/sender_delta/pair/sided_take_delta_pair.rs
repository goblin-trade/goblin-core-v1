use crate::{
    axis::leg::{Base, Pair, Quote},
    settlement::{sender_delta::SidedTakeDeltaV2, ConstZero},
};

pub type SidedTakeDeltaPairV2 = Pair<SidedTakeDeltaV2<Base>, SidedTakeDeltaV2<Quote>>;

impl ConstZero for SidedTakeDeltaPairV2 {
    const ZEROED: Self = Pair::new(
        SidedTakeDeltaV2::<Base>::ZEROED,
        SidedTakeDeltaV2::<Quote>::ZEROED,
    );
}
