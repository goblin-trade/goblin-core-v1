use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{
            token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
            ETH,
        },
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::SenderTokenStore, local_delta::Deposits, CheckedAdd, Delta,
        SidedSenderDeltaV2, UnsideDelta, UnsidedSenderDeltaV2,
    },
    types::StoreReader,
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ();
    type Address = ();
    type Deposit = ();
    // type Delta = EthDelta;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }

    // Stub. ETH cannot be deposited.
    fn set_deposit<In: LegMatcher>(_deposits: &mut Deposits, _deposit_amount: Self::Deposit) {}

    fn get_global_delta<In>(
        _token_index: Self::TokenIndex,
        delta: &mut Delta,
    ) -> Result<&mut SenderTokenStore<Self>, GoblinError>
    where
        In: LegMatcher,
    {
        let store = Self::get_leg_mut(&mut delta.global.global_sender_delta);
        Ok(store)
    }

    fn commit_sender_delta<In>(
        _token_index: Self::TokenIndex,
        lot_size_pair: &LotSizePair,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        let eth_delta = Self::get_leg_mut(&mut delta.global.global_sender_delta);

        // 1. Credit deposit amount- stub. ETH cannot be deposited in the middle

        // 2. Credit sender delta
        let local_sender_delta = In::get_leg(&delta.local.local_sender_delta);
        let unsided_sender_delta = local_sender_delta.unside(lot_size_pair);

        eth_delta.unsided_sender_delta = eth_delta
            .unsided_sender_delta
            .checked_add(unsided_sender_delta)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
