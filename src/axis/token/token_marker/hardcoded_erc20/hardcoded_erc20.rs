use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
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
    quantities::{DeltaAtoms, DeltaLots},
    settlement::{global_delta::SenderTokenStore, Delta, Settleable},
    types::{Address, StoreReader},
};

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;
    type Address = Address;

    type LocalDeposit = DeltaLots;
    type GlobalDeposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = HARDCODED_TOKENS
            .get(token_index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)?;

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

    fn settle_deposit(
        deposit: Self::GlobalDeposit,
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
        msg_sender: &Address,
    ) -> Result<(), GoblinError> {
        let token_address = Self::token_index_to_address(token_index, custom_erc20_list)?;

        Ok(())
        // deposit.settle(&token_address, msg_sender)
    }
}
