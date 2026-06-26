use crate::{axis::leg::Pair, quantities::DeltaLots, settlement::ConstZero};

pub type DeltaLotsPair = Pair<DeltaLots, DeltaLots>;

impl ConstZero for DeltaLotsPair {
    const ZEROED: Self = Pair::new(DeltaLots::ZEROED, DeltaLots::ZEROED);
}
