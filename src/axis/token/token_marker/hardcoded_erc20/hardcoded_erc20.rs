use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{
            token_marker::{
                custom_erc20::custom_erc20_data::CustomERC20Data,
                hardcoded_erc20::{hardcoded_erc20_index::HardcodedERC20Index, HARDCODED_TOKENS},
                TokenMarker,
            },
            HardcodedERC20,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{
        local_delta::Deposits, CheckedAdd, Delta, SidedSenderDeltaV2, UnsideDelta,
        UnsidedSenderDeltaV2,
    },
    types::{Address, StoreReader},
};

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = HARDCODED_TOKENS
            .get(token_index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)?;

        Ok(data.address)
    }

    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher,
    {
        *In::get_leg_mut(deposits) = deposit_amount;
    }

    fn commit_sender_delta<In>(
        delta: &mut Delta,
        token_index: Self::TokenIndex,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        let local_sender_delta = In::get_leg(&delta.local.local_sender_delta);
        let unsided_sender_delta = local_sender_delta.unside(lot_size_pair);

        let delta_list = Self::get_leg_mut(&mut delta.global.global_sender_delta);
        let erc20_delta = &mut delta_list[token_index.0];

        // Credit deposit amount
        let deposit = In::get(&delta.local.deposits);

        erc20_delta.deposit_due = erc20_delta
            .deposit_due
            .checked_add(deposit)
            .ok_or(GoblinError::Overflow)?;

        // Credit sender delta
        erc20_delta.unsided_sender_delta = erc20_delta
            .unsided_sender_delta
            .checked_add(unsided_sender_delta)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
