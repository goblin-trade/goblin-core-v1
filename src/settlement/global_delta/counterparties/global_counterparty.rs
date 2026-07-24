use crate::{
    axis::{
        leg::SamePair,
        update::{SameUpdatePair, UpdatePair},
    },
    quantities::UnsidedAtoms,
    settlement::ConstZero,
};

pub type GlobalCounterparty = SameUpdatePair<UnsidedAtoms>;

impl ConstZero for GlobalCounterparty {
    const ZEROED: Self = UpdatePair::new(UnsidedAtoms::ZEROED, UnsidedAtoms::ZEROED);
}

impl ConstZero for SamePair<GlobalCounterparty> {
    const ZEROED: Self = SamePair::new(GlobalCounterparty::ZEROED, GlobalCounterparty::ZEROED);
}
