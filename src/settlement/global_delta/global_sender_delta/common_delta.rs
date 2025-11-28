use crate::{
    markets::LotSizePair,
    quantities::{AsUnsided, QuantityOps, UnsidedAtoms},
    settlement::{
        global_delta::GlobalUpdate,
        local_delta::{LocalSenderDelta, TakerDelta},
    },
    types::{Base, LegMarker, PairAccessor, Quote},
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
    pub fn new<In>(local_sender_delta: &LocalSenderDelta, lot_size_pair: &LotSizePair) -> Self
    where
        In: LegMarker
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
        In::Opposite:
            PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In::Opposite>>,
    {
        let global_update =
            GlobalUpdate::<In>::new(&local_sender_delta.taker_delta_pair, lot_size_pair);

        Self {
            taker_in: global_update.taker_in.unsided(),
            taker_out: global_update.taker_out.unsided(),
            taker_self_trade_unlocked: global_update.taker_self_trade_unlocked.unsided(),
            ..Default::default()
        }
    }

    pub fn apply_local_update<In>(
        &mut self,
        local_sender_delta: &LocalSenderDelta,
        lot_size_pair: &LotSizePair,
    ) -> Option<()>
    where
        In: LegMarker
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In>>,
        In::Opposite:
            PairAccessor<TakerDelta<Base>, TakerDelta<Quote>, Result = TakerDelta<In::Opposite>>,
    {
        let global_update =
            GlobalUpdate::<In>::new(&local_sender_delta.taker_delta_pair, lot_size_pair);

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
