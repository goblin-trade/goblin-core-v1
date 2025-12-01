use crate::{
    quantities::{AsUnsided, QuantityOps, UnsidedAtoms},
    settlement::global_delta::GlobalUpdate,
    types::LegMarker,
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
pub struct CommonDelta {
    /// Tokens transferred into the engine, i.e. lost as taker
    pub taker_in: UnsidedAtoms,

    /// Tokens transferred out by the engine, i.e. gained as taker
    pub taker_out: UnsidedAtoms,

    /// Locked tokens released from taking a self trade
    pub taker_self_trade_unlocked: UnsidedAtoms,

    /// Tokens locked on making a resting order
    pub maker_locked: UnsidedAtoms,

    /// Tokens unlocked on cancelling a resting order
    pub cancel_unlocked: UnsidedAtoms,
}

impl CommonDelta {
    pub const fn new() -> Self {
        Self {
            taker_in: UnsidedAtoms::ZERO,
            taker_out: UnsidedAtoms::ZERO,
            taker_self_trade_unlocked: UnsidedAtoms::ZERO,
            maker_locked: UnsidedAtoms::ZERO,
            cancel_unlocked: UnsidedAtoms::ZERO,
        }
    }

    pub fn add_global_update<In: LegMarker>(
        &mut self,
        global_update: &GlobalUpdate<In>,
    ) -> Option<()> {
        self.taker_in = self
            .taker_in
            .checked_add(global_update.taker_in.unsided())?;
        self.taker_out = self
            .taker_out
            .checked_add(global_update.taker_out.unsided())?;
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
