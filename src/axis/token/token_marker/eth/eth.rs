use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
            ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{global_delta::SenderTokenStore, local_delta::Deposits, Delta},
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
    fn set_local_deposit<In: LegMatcher>(_deposits: &mut Deposits, _deposit_amount: Self::Deposit) {
    }

    // Stub. ETH cannot be deposited.
    fn add_global_deposit<In>(
        _deposit: DeltaAtoms,
        _global_delta: &mut SenderTokenStore<Self>,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        Ok(())
    }

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
}
