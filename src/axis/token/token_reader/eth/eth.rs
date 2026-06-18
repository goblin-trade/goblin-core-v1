use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_reader::{
                custom_erc20::custom_erc20_data::CustomERC20Data, eth::ETHStub, TokenReader,
            },
            ETH,
        },
    },
    goblin_error::GoblinError,
    settlement::{global_delta::SenderTokenStore, Delta},
    types::StoreReader,
};

impl TokenReader for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ETHStub;
    type Address = ETHStub;
    type Deposit = ETHStub;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(ETHStub)
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
