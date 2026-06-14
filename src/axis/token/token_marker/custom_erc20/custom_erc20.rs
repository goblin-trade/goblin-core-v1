use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_marker::{
                custom_erc20::{
                    custom_erc20_data::CustomERC20Data, custom_erc20_index::CustomERC20Index,
                },
                TokenMarker,
            },
            CustomERC20,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{global_delta::SenderTokenStore, local_delta::Deposits, Delta},
    types::{Address, StoreReader},
};

impl TokenMarker for CustomERC20 {
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

    fn set_local_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher,
    {
        *In::get_leg_mut(deposits) = deposit_amount;
    }

    fn add_global_deposit<In>(
        deposit: DeltaAtoms,
        global_delta: &mut SenderTokenStore<Self>,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
    {
        global_delta.deposit_due = global_delta
            .deposit_due
            .checked_add(deposit)
            .ok_or(GoblinError::Overflow)?;
        Ok(())
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
