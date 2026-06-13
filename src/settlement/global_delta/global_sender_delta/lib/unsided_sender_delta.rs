use crate::{
    axis::leg::leg_matcher::LegMatcher,
    quantities::{QuantityOps, UnsidedAtoms},
};

// /// Common delta shared by ETHDelta and ERC20Delta
// ///
// /// # Note on locked tokens
// ///
// /// * Unlocked tokens must be credited to free tokens and locked tokens must
// /// be debited from free.
// ///
// /// * To avoid creating invalid states, we only update the locked and unlocked
// /// accumulators here. By definition, `maker_locked` cannot change `taker_out`.
// ///
// /// * Unlocked tokens are credited to free tokens in settlement phase.
// #[derive(Clone, Copy)]
// pub struct UnsidedSenderDelta {
//     pub matched_unsided_atoms: MatchedUnsidedAtoms,

//     /// Tokens locked on making a resting order
//     pub make_locked: UnsidedAtoms,

//     /// Tokens unlocked on cancelling a resting order
//     pub reduce_unlocked: UnsidedAtoms,
// }

// impl UnsidedSenderDelta {
//     pub const fn zero() -> Self {
//         Self {
//             matched_unsided_atoms: MatchedUnsidedAtoms::zero(),
//             make_locked: UnsidedAtoms::ZERO,
//             reduce_unlocked: UnsidedAtoms::ZERO,
//         }
//     }

//     pub fn add_global_update<In: LegMatcher>(
//         &mut self,
//         global_update: &GlobalSenderUpdate<In>,
//     ) -> Option<()> {
//         let matched_unsided_atoms = MatchedUnsidedAtoms::from(&global_update.matched_atoms);

//         self.matched_unsided_atoms
//             .checked_add(matched_unsided_atoms)?;

//         Some(())
//     }

//     // pub fn free_atoms_out(&self) -> Option<UnsidedAtoms> {
//     //     self.taker_out
//     //         .checked_add(self.taker_self_trade_unlocked)?
//     //         .checked_add(self.cancel_unlocked)
//     // }

//     // pub fn free_atoms_in(&self) -> Option<UnsidedAtoms> {
//     //     self.taker_in.checked_add(self.maker_locked)
//     // }
// }
