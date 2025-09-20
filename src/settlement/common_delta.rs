use crate::{
    goblin_error::GoblinError,
    markets::{indexed_market, IndexedMarket},
    matching::MatchResult,
    quantities::Atoms,
    settlement::MarketSenderDelta,
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
    pub taker_in: Atoms,

    /// Tokens transferred out by the engine, i.e. gained as taker
    pub taker_out: Atoms,

    /// Locked tokens released from taking a self trade
    pub taker_self_trade_unlocked: Atoms,

    /// Tokens locked on making a resting order
    pub maker_locked: Atoms,

    /// Tokens unlocked on cancelling a resting order
    pub cancel_unlocked: Atoms,
}

impl CommonDelta {
    pub fn free_atoms_out(&self) -> Result<Atoms, GoblinError> {
        self.taker_out
            .checked_add(self.taker_self_trade_unlocked)?
            .checked_add(self.cancel_unlocked)
    }

    pub fn free_atoms_in(&self) -> Result<Atoms, GoblinError> {
        self.taker_in.checked_add(self.maker_locked)
    }

    // Update the amounts of the token transferred in and transferred out in a market
    //
    // # Arguments
    // * taker_in: Atoms transferred in on a taker trade for side In
    // * taker_out: Atoms transferred out on opposite side trade for side In::Opposite
    //
    pub fn accumulate_market_delta(
        &mut self,
        taker_in: Atoms,
        taker_out: Atoms,
        taker_self_trade: Atoms,
    ) -> Result<(), GoblinError> {
        self.taker_in = self.taker_in.checked_add(taker_in)?;
        self.taker_out = self.taker_out.checked_add(taker_out)?;
        self.taker_self_trade_unlocked = self
            .taker_self_trade_unlocked
            .checked_add(taker_self_trade)?;

        Ok(())
    }
}
