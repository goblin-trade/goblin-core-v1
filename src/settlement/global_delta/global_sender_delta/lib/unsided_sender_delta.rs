use crate::{
    quantities::{AsUnsided, QuantityOps, UnsidedAtoms},
    settlement::{global_delta::GlobalSenderUpdate, MatchedUnsidedAtoms},
    types::LegMatcher,
};

/// Common delta shared by ETHDelta and ERC20Delta
///
/// # Note on locked tokens
///
/// * Unlocked tokens must be credited to free tokens and locked tokens must
/// be debited from free.
///
/// * To avoid creating invalid states, we only update the locked and unlocked
/// accumulators here. By definition, `maker_locked` cannot change `taker_out`.
///
/// * Unlocked tokens are credited to free tokens in settlement phase.
#[derive(Clone, Copy)]
pub struct UnsidedSenderDelta {
    pub matched_unsided_atoms: MatchedUnsidedAtoms,

    /// Locked tokens released from taking a self trade
    pub taker_self_trade_unlocked: UnsidedAtoms,

    /// Tokens locked on making a resting order
    pub maker_locked: UnsidedAtoms,

    /// Tokens unlocked on cancelling a resting order
    pub cancel_unlocked: UnsidedAtoms,
}

impl UnsidedSenderDelta {
    pub const fn zero() -> Self {
        Self {
            matched_unsided_atoms: MatchedUnsidedAtoms::zero(),
            taker_self_trade_unlocked: UnsidedAtoms::ZERO,
            maker_locked: UnsidedAtoms::ZERO,
            cancel_unlocked: UnsidedAtoms::ZERO,
        }
    }

    pub fn add_global_update<In: LegMatcher>(
        &mut self,
        global_update: &GlobalSenderUpdate<In>,
    ) -> Option<()> {
        let matched_unsided_atoms = MatchedUnsidedAtoms::from(&global_update.matched_atoms);

        self.matched_unsided_atoms
            .checked_add(matched_unsided_atoms)?;

        self.taker_self_trade_unlocked = self
            .taker_self_trade_unlocked
            .checked_add(global_update.taker_self_trade_unlocked.unsided())?;

        Some(())
    }

    // pub fn free_atoms_out(&self) -> Option<UnsidedAtoms> {
    //     self.taker_out
    //         .checked_add(self.taker_self_trade_unlocked)?
    //         .checked_add(self.cancel_unlocked)
    // }

    // pub fn free_atoms_in(&self) -> Option<UnsidedAtoms> {
    //     self.taker_in.checked_add(self.maker_locked)
    // }
}
