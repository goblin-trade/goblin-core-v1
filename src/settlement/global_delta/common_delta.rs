use crate::{
    markets::LotSizePair,
    quantities::{QuantityOps, UnsidedAtoms},
    settlement::local_delta::LocalSenderDelta,
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
#[derive(Default, Clone, Copy)]
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
    pub fn free_atoms_out(&self) -> Option<UnsidedAtoms> {
        self.taker_out
            .checked_add(self.taker_self_trade_unlocked)?
            .checked_add(self.cancel_unlocked)
    }

    pub fn free_atoms_in(&self) -> Option<UnsidedAtoms> {
        self.taker_in.checked_add(self.maker_locked)
    }

    pub fn apply_local_update<In>(
        &mut self,
        local_sender_delta: &LocalSenderDelta,
        lot_size_pair: &LotSizePair,
    ) where
        In: LegMarker,
    {
    }

    // // Update the amounts of the token transferred in and transferred out in a market
    // // when performing bid and ask taker orders
    // pub fn apply_taker_update(&mut self, taker_token_update: &TakerTokenUpdate) -> Option<()> {
    //     self.taker_in = self
    //         .taker_in
    //         .checked_add(taker_token_update.free_atoms_in)?;
    //     self.taker_out = self
    //         .taker_out
    //         .checked_add(taker_token_update.locked_atoms_out)?;
    //     self.taker_self_trade_unlocked = self
    //         .taker_self_trade_unlocked
    //         .checked_add(taker_token_update.atoms_released_by_self_trade)?;

    //     Some(())
    // }
}
