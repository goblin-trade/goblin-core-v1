use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    settlement::{CheckedAdd, Delta, SidedSenderDeltaV2, UnsideDelta, UnsidedSenderDeltaV2},
};

pub trait TokenDeltaManager: TokenMarker {
    fn commit_sender_delta<In>(
        token_index: Self::TokenIndex,
        lot_size_pair: &LotSizePair,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        let leg_deposit = In::get_leg(&delta.local.deposits);
        let deposit = Self::get(leg_deposit);

        let local_sender_delta = In::get_leg(&delta.local.local_sender_delta);
        let unsided_sender_delta = local_sender_delta.unside(lot_size_pair);

        let global_delta = Self::get_global_delta::<In>(token_index, delta)?;

        // 1. Add deposit
        global_delta.deposit_due = global_delta
            .deposit_due
            .checked_add(deposit)
            .ok_or(GoblinError::Overflow)?;

        // 2. Add sender delta
        global_delta.unsided_sender_delta = global_delta
            .unsided_sender_delta
            .checked_add(unsided_sender_delta)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}

impl TokenDeltaManager for ETH {}
impl TokenDeltaManager for HardcodedERC20 {}
impl TokenDeltaManager for CustomERC20 {}
