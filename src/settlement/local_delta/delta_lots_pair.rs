use crate::{axis::leg::Pair, quantities::UnsidedDeltaLots, settlement::ConstZero};

pub type DeltaLotsPair = Pair<UnsidedDeltaLots, UnsidedDeltaLots>;

impl ConstZero for DeltaLotsPair {
    const ZEROED: Self = Pair::new(UnsidedDeltaLots::ZEROED, UnsidedDeltaLots::ZEROED);
}
