use crate::{
    axis::leg::{Pair, SamePair},
    quantities::DeltaAtoms,
    settlement::ConstZero,
};

pub type DeltaAtomsPair = SamePair<DeltaAtoms>;

impl ConstZero for DeltaAtomsPair {
    const ZEROED: Self = Pair::new(DeltaAtoms::ZEROED, DeltaAtoms::ZEROED);
}
