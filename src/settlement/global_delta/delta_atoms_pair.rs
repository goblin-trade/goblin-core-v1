use crate::{
    axis::leg::{Pair, SamePair},
    quantities::UnsidedDeltaAtoms,
    settlement::ConstZero,
};

pub type DeltaAtomsPair = SamePair<UnsidedDeltaAtoms>;

impl ConstZero for DeltaAtomsPair {
    const ZEROED: Self = Pair::new(UnsidedDeltaAtoms::ZEROED, UnsidedDeltaAtoms::ZEROED);
}
