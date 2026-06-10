use crate::{
    axis::leg::leg_math::LegMath,
    quantities::UnsidedAtoms,
    settlement::{sender_delta::TakeDelta, SidedTakeDeltaV2},
};

pub type UnsidedTakeDeltaV2 = TakeDelta<UnsidedAtoms, UnsidedAtoms>;

// impl<In> From<(SidedTakeDeltaV2<In>)> for UnsidedTakeDeltaV2
// where
//     In: LegMath,
// {
//     fn from(value: (SidedTakeDeltaV2<In>)) -> Self {
//         todo!()
//     }
// }
