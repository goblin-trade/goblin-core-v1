use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
            ETH,
        },
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::{EthDelta, GlobalSenderDelta},
        local_delta::Deposits,
    },
    types::StoreReader,
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ();
    type Address = ();
    type Deposit = ();
    type Delta = EthDelta;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }

    // Stub. ETH cannot be deposited.
    fn set_deposit<In: LegMatcher>(_deposits: &mut Deposits, _deposit_amount: Self::Deposit) {}

    fn add_delta(
        delta: Self::Delta,
        token_index: Self::TokenIndex,
        global_sender_delta: &mut GlobalSenderDelta,
    ) {
        let store = Self::get_leg_mut(global_sender_delta);
        store.unsided_sender_delta.matched_unsided_atoms;
    }
}
