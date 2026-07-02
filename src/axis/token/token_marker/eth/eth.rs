use crate::{
    axis::token::{
        token_marker::{
            custom_erc20::custom_erc20_list::CustomERC20List, eth::ETHStub, TokenMarker,
        },
        ETH,
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::StoreReader,
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ETHStub;
    type TokenAddress = ETHStub;
    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError> {
        Ok(ETHStub)
    }

    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    fn get_global_token_delta(
        _token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self> {
        Self::get_leg_mut(global_sender)
    }
}
