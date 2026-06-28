use crate::{
    axis::token::{
        token_marker::{
            custom_erc20::{
                custom_erc20_data::CustomERC20Data, custom_erc20_index::CustomERC20Index,
            },
            TokenMarker,
        },
        CustomERC20,
    },
    goblin_error::GoblinError,
    quantities::{DeltaAtoms, DeltaLots, UnsidedDeltaAtomsPerLot},
    settlement::global_delta_v3::{GlobalSender, TokenDeltaV3},
    types::{Address, StoreReader},
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;
    type Address = Address;

    type LocalDeposit = DeltaLots;
    type GlobalDeposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = custom_erc20_list
            .get(token_index.0)
            .ok_or(GoblinError::InvalidCustomTokenIndex)?;
        Ok(data.address)
    }

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn get_token_delta_v3(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDeltaV3<Self> {
        let list = Self::get_leg_mut(global_sender);
        let token_delta = &mut list[token_index.0];
        token_delta
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
