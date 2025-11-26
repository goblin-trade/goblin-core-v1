use crate::{
    markets::LotSizePair,
    matching::MatchResult,
    quantities::{AsUnsided, QuantityOps, UnsidedAtoms},
    settlement::local_delta::LocalSenderDelta,
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
    ) -> Option<()>
    where
        In: LegMarker
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In>>,
        In::Opposite:
            PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In::Opposite>>,
    {
        let base_lot_size = lot_size_pair.base;
        let lot_size = *In::get_leg(&lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);

        let delta = In::get_leg(&local_sender_delta.take_result_pair);
        let delta_opposite = In::Opposite::get_leg(&local_sender_delta.take_result_pair);

        let free_atoms_in =
            In::matching_lots_to_atoms(delta.free_matching_lots_in, base_lot_size, atoms_per_lot);

        let locked_atoms_out = In::matching_lots_to_atoms(
            delta_opposite.locked_matching_lots_out,
            base_lot_size,
            atoms_per_lot,
        );

        let atoms_released_by_self_trade = In::matching_lots_to_atoms(
            delta_opposite.released_by_self_trade,
            base_lot_size,
            atoms_per_lot,
        );

        self.taker_in = self.taker_in.checked_add(free_atoms_in.unsided())?;
        self.taker_out = self.taker_out.checked_add(locked_atoms_out.unsided())?;
        self.taker_self_trade_unlocked = self
            .taker_self_trade_unlocked
            .checked_add(atoms_released_by_self_trade.unsided())?;

        Some(())
    }
}
