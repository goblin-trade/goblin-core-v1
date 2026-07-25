use crate::{
    axis::update::{SameUpdatePair, UpdatePair},
    quantities::UnsidedAtoms,
    settlement::ConstZero,
};

pub type GlobalCounterparty = SameUpdatePair<UnsidedAtoms>;

impl ConstZero for GlobalCounterparty {
    const ZEROED: Self = UpdatePair::new(UnsidedAtoms::ZEROED, UnsidedAtoms::ZEROED);
}
