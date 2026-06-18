use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_reader::{
                custom_erc20::{
                    custom_erc20_data::CustomERC20Data, custom_erc20_index::CustomERC20Index,
                },
                TokenReader,
            },
            CustomERC20,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{global_delta::SenderTokenStore, Delta},
    types::{Address, StoreReader},
};

impl TokenReader for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = custom_erc20_list
            .get(token_index.0)
            .ok_or(GoblinError::InvalidCustomTokenIndex)?;
        Ok(data.address)
    }

    fn get_global_delta<In>(
        token_index: Self::TokenIndex,
        delta: &mut Delta,
    ) -> Result<&mut SenderTokenStore<Self>, GoblinError>
    where
        In: LegMatcher,
    {
        let list = Self::get_leg_mut(&mut delta.global.global_sender_delta);
        let store = &mut list[token_index.0];
        Ok(store)
    }
}
