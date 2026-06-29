use crate::{
    axis::token::{
        token_marker::{
            custom_erc20::custom_erc20_list::CustomERC20List,
            hardcoded_erc20::{hardcoded_erc20_index::HardcodedERC20Index, HARDCODED_TOKENS},
            TokenMarker,
        },
        HardcodedERC20,
    },
    goblin_error::GoblinError,
    quantities::{DeltaAtoms, DeltaLots, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::{Address, StoreReader},
};

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;
    type TokenAddress = Address;
    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = DeltaLots;
    type GlobalDeposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        let data = HARDCODED_TOKENS
            .get(token_index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)?;

        Ok(data.address)
    }

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn get_global_token_delta(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self> {
        let list = Self::get_leg_mut(global_sender);
        let token_delta = &mut list[token_index.0];
        token_delta
    }

    fn settle_deposit(
        deposit: Self::GlobalDeposit,
        token_index: Self::TokenIndex,
        custom_erc20_list: CustomERC20List,
        msg_sender: &Address,
    ) -> Result<(), GoblinError> {
        let token_address = Self::token_index_to_address(token_index, custom_erc20_list)?;

        Ok(())
        // deposit.settle(&token_address, msg_sender)
    }
}
