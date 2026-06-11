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
        global_delta::{EthDelta, GlobalDelta, GlobalSenderDelta},
        local_delta::Deposits,
        Delta, SidedMakeDeltaV2, SidedSenderDeltaV2, SidedTakeDeltaV2, UnsideDelta,
        UnsidedMakeDeltaV2, UnsidedSenderDeltaV2, UnsidedTakeDeltaV2,
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

    fn add_delta<In>(
        delta: &mut Delta,
        lot_size_pair: &LotSizePair, // delta: Self::Delta,
                                     // token_index: Self::TokenIndex,
                                     // global_sender_delta: &mut GlobalSenderDelta,
    ) where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        let global_eth_delta = Self::get_leg_mut(&mut delta.global.global_sender_delta);
        let local_sender_delta = In::get_leg(&delta.local.local_sender_delta);

        let unsided_sender_delta = local_sender_delta.unside(lot_size_pair);

        // TODO add to global
    }
}
