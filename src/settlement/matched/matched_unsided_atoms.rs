use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{QuantityOps, UnsideQuantity, UnsidedAtoms},
    settlement::{CheckedAdd, MatchedAtoms},
};

// #[derive(Default, Clone, Copy, PartialEq)]
// pub struct MatchedUnsidedAtoms {
//     /// Atoms traded in by taker and gained by maker
//     pub taker_in: UnsidedAtoms,

//     /// Atoms obtained by taker and lost by maker
//     pub taker_out: UnsidedAtoms,
// }

// impl<In: LegMatcher> From<&MatchedAtoms<In>> for MatchedUnsidedAtoms {
//     fn from(value: &MatchedAtoms<In>) -> Self {
//         Self {
//             taker_in: value.taker_in.unsided(),
//             taker_out: value.taker_out.unsided(),
//         }
//     }
// }

// impl MatchedUnsidedAtoms {
//     pub const fn zero() -> Self {
//         Self {
//             taker_in: UnsidedAtoms::ZERO,
//             taker_out: UnsidedAtoms::ZERO,
//         }
//     }

//     pub fn checked_add(&mut self, rhs: Self) -> Option<()> {
//         self.taker_in = self.taker_in.checked_add(rhs.taker_in)?;
//         self.taker_out = self.taker_out.checked_add(rhs.taker_out)?;

//         Some(())
//     }
// }
